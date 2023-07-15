use anyhow::{anyhow, Result};
use reqwest::{self, Client, Url};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::Write,
    time::{Duration, Instant},
};

const MANIFEST_URL: &str = "https://releases.aosc.io/manifest/recipe.json";
const IS_RETRO: bool = cfg!(feature = "is_retro");
const SPEEDTEST_FILE_CHECKSUM: &str =
    "30e14955ebf1352266dc2ff8067e68104607e750abb9d3b36582b8af909fcb58";

#[macro_export]
macro_rules! DEPLOYKIT_USER_AGENT {
    () => {
        format!("AOSC DeployKit/{}", env!("CARGO_PKG_VERSION"))
    };
}

// mirror manifests
#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Mirror {
    pub name: String,
    #[serde(rename = "name-tr")]
    pub name_tr: String,
    pub loc: String,
    #[serde(rename = "loc-tr")]
    pub loc_tr: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tarball {
    pub arch: String,
    pub date: String,
    #[serde(rename = "downloadSize")]
    pub download_size: i64,
    #[serde(rename = "instSize")]
    pub inst_size: i64,
    pub path: String,
    pub sha256sum: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Squashfs {
    pub arch: String,
    pub date: String,
    #[serde(rename = "downloadSize")]
    pub download_size: i64,
    #[serde(rename = "instSize")]
    pub inst_size: i64,
    pub path: String,
    pub sha256sum: String,
    pub inodes: u32,
}

#[derive(Deserialize, Debug)]
struct SystemRootFs {
    arch: String,
    date: String,
    download_size: i64,
    inst_size: i64,
    path: String,
    sha256sum: String,
    // inodes: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Variant {
    name: String,
    retro: bool,
    pub description: String,
    #[serde(rename = "description-tr")]
    pub description_tr: String,
    tarball: Vec<SystemRootFs>,
    squashfs: Vec<SystemRootFs>,
}

#[derive(Deserialize, Debug)]
pub struct Bulletin {
    #[serde(rename = "type")]
    pub type_: String,
    pub title: String,
    #[serde(rename = "title-tr")]
    pub title_tr: String,
    pub body: String,
    #[serde(rename = "body-tr")]
    pub body_tr: String,
}

#[derive(Deserialize, Debug)]
pub struct Recipe {
    pub version: usize,
    pub bulletin: Bulletin,
    variants: Vec<Variant>,
    mirrors: Vec<Mirror>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VariantEntry {
    pub name: String,
    pub size: u64,
    pub install_size: u64,
    pub date: String,
    pub sha256sum: String,
    pub url: String,
}

pub fn fetch_recipe() -> Result<Recipe> {
    Ok(reqwest::blocking::get(MANIFEST_URL)?.json()?)
}

pub fn fetch_mirrors(recipe: &Recipe) -> Vec<Mirror> {
    recipe.mirrors.clone()
}

pub fn get_variants() -> Result<Vec<VariantEntry>> {
    let recipe = fetch_recipe()?;
    let variants = find_variant_candidates(recipe)?;

    Ok(variants)
}

/// AOSC OS specific architecture mapping for ppc64
#[cfg(target_arch = "powerpc64")]
#[inline]
pub(crate) fn get_arch_name() -> Option<&'static str> {
    use nix::libc;
    let mut endian: libc::c_int = -1;
    let result;
    unsafe {
        result = libc::prctl(libc::PR_GET_ENDIAN, &mut endian as *mut libc::c_int);
    }
    if result < 0 {
        return None;
    }
    match endian {
        libc::PR_ENDIAN_LITTLE | libc::PR_ENDIAN_PPC_LITTLE => Some("ppc64el"),
        libc::PR_ENDIAN_BIG => Some("ppc64"),
        _ => None,
    }
}

/// AOSC OS specific architecture mapping table
#[cfg(not(target_arch = "powerpc64"))]
#[inline]
pub(crate) fn get_arch_name() -> Option<&'static str> {
    use std::env::consts::ARCH;
    match ARCH {
        "x86_64" => Some("amd64"),
        "x86" => Some("i486"),
        "powerpc" => Some("powerpc"),
        "aarch64" => Some("arm64"),
        "mips64" => Some("loongson3"),
        "riscv64" => Some("riscv64"),
        "loongarch64" => Some("loongarch64"),
        _ => None,
    }
}

/// Issue a HEAD request to the specified url instead of downloading the entire body.
///
/// If the server returned a error code the response becomes an error.
pub fn query_file_meta(url: &String) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(DEPLOYKIT_USER_AGENT!())
        .build()?;
    let head_response = client.head(url).send();

    let server_response = head_response.map_err(|e| anyhow!("{}", e))?;
    let server_success = server_response
        .error_for_status()
        .map_err(|e| anyhow!("{}", e))?;

    Ok(server_success)
}

pub fn speedtest_mirrors(mirrors: Vec<Mirror>) -> Vec<Mirror> {
    let mut speedtest_mirror = vec![];
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        .unwrap();
    let client = reqwest::Client::builder()
        .user_agent(DEPLOYKIT_USER_AGENT!())
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    runtime.block_on(async move {
        let mut task = vec![];
        for mirror in &mirrors {
            task.push(get_mirror_speed_score(&mirror.url, &client))
        }
        let results = futures::future::join_all(task).await;
        for (index, result) in results.into_iter().enumerate() {
            if let Ok(score) = result {
                speedtest_mirror.push((mirrors[index].loc_tr.to_owned(), score));
            }
        }
        speedtest_mirror.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        let mut new_mirrors = vec![];
        for (name, _) in speedtest_mirror {
            let index = mirrors.iter().position(|x| x.loc_tr == name).unwrap();
            new_mirrors.push(mirrors[index].to_owned());
        }
        new_mirrors
    })
}

async fn get_mirror_speed_score(mirror_url: &str, client: &Client) -> Result<f32> {
    let download_url = Url::parse(mirror_url)?.join("../.repotest")?;
    let timer = Instant::now();
    let file = client.get(download_url).send().await?.bytes().await?;
    let mut hasher = Sha256::new();
    hasher.write_all(&file)?;

    if hex::encode(hasher.finalize()) == SPEEDTEST_FILE_CHECKSUM {
        let result_time = timer.elapsed().as_secs_f32();
        return Ok(result_time);
    }

    Err(anyhow!(
        "Installer failed benchmark {}, please check your network connection!",
        mirror_url
    ))
}

pub fn find_variant_candidates(recipes: Recipe) -> Result<Vec<VariantEntry>> {
    let mut results: Vec<VariantEntry> = Vec::new();
    let arch_name = get_arch_name();
    if arch_name.is_none() {
        return Err(anyhow!("Unsupported architecture."));
    }
    let arch_name = arch_name.unwrap();
    // filter: tarballs array is not empty and the mainline/retro switch matches
    let mut all_empty = true;
    let right_recipes = recipes
        .variants
        .into_iter()
        .filter(|x| x.retro == IS_RETRO && !x.squashfs.is_empty() && x.name != "BuildKit")
        .collect::<Vec<Variant>>();

    let right_recipes_len = right_recipes.len();
    for (index, recipe) in right_recipes.into_iter().enumerate() {
        let rootfs = match IS_RETRO {
            true => recipe.tarball,
            false => recipe.squashfs,
        };

        let mut sorted_rootfs: Vec<SystemRootFs> =
            rootfs.into_iter().filter(|x| x.arch == arch_name).collect();

        sorted_rootfs.sort_by(|a, b| b.date.cmp(&a.date));

        if sorted_rootfs.is_empty() {
            if all_empty && index == right_recipes_len - 1 {
                return Err(anyhow!(
                    "Installer could not find any available system release for your device."
                ));
            }
            continue;
        }
        all_empty = false;

        let candidate_rootfs = sorted_rootfs.first().unwrap();
        results.push(VariantEntry {
            name: recipe.name,
            size: candidate_rootfs.download_size as u64,
            install_size: candidate_rootfs.inst_size as u64,
            date: candidate_rootfs.date.clone(),
            url: candidate_rootfs.path.clone(),
            sha256sum: candidate_rootfs.sha256sum.clone(),
        });
    }
    results.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(results)
}
