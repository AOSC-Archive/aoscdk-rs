use anyhow::{anyhow, Result};
use reqwest::{self, Url};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env::consts::ARCH,
    time::{Duration, Instant},
};

const MANIFEST_URL: &str = "https://releases.aosc.io/manifest/recipe.json";
const IS_RETRO: bool = cfg!(feature = "is_retro");
const SPEEDTEST_FILE_CHECKSUM: &str = "98900564fb4d9c7d3b63f44686c5b8a120af94a51fc6ca595e1406d5d8cc0416";
const USER_AGENT: &str = "Mozilla/5.0 (X11; AOSC OS; Linux x86_64; rv:93.0) Gecko/20100101 Firefox/93.0";

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

#[derive(Deserialize)]
pub struct Variant {
    name: String,
    retro: bool,
    pub description: String,
    #[serde(rename = "description-tr")]
    pub description_tr: String,
    tarballs: Vec<Tarball>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Recipe {
    pub version: usize,
    pub bulletin: Bulletin,
    variants: Vec<Variant>,
    mirrors: Vec<Mirror>,
}

#[derive(Deserialize, Debug)]
struct DistroDownload {
    name: String,
    url: String,
}
#[derive(Deserialize, Debug)]
struct DistroData {
    name: String,
    description: String,
    downloads: Vec<DistroDownload>,
}
#[derive(Deserialize, Debug)]
struct DistroList {
    list: Vec<DistroData>,
}
#[derive(Deserialize, Debug)]
struct VariantData {
    general: DistroList,
    retro: DistroList,
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

#[inline]
fn get_arch_name() -> Option<&'static str> {
    match ARCH {
        "x86_64" => Some("amd64"),
        "x86" => Some("i486"),
        "powerpc" => Some("powerpc"),
        "aarch64" => Some("arm64"),
        "mips64" => Some("loongson3"),
        _ => None,
    }
}

pub fn download_file(url: &str) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()?;
    let resp = client.get(url).send()?;
    let resp = resp.error_for_status()?;

    Ok(resp)
}

pub fn speedtest_mirrors(mirrors: Vec<Mirror>) -> Vec<Mirror> {
    let mut speedtest_mirror = vec![];
    for mirror in &mirrors {
        let score;
        match get_mirror_speed_score(&mirror.url) {
            Ok(s) => score = s,
            Err(_) => {
                continue
            },
        }
        let name = &mirror.loc_tr;
        speedtest_mirror.push((name, score));
    }
    speedtest_mirror.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    let mut new_mirrors = vec![];
    for (name, _) in speedtest_mirror {
        let index = mirrors.iter().position(|x| &x.loc_tr == name).unwrap();
        new_mirrors.push(mirrors[index].to_owned());
    }

    new_mirrors
}

fn get_mirror_speed_score(mirror_url: &str) -> Result<f32> {
    let download_url = Url::parse(mirror_url)?.join("../misc/u-boot-sunxi-with-spl.bin")?;
    let client = reqwest::blocking::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .timeout(Duration::from_secs(10))
        .build()?;
    let timer = Instant::now();
    let mut file = client.get(download_url).send()?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let result_time = timer.elapsed().as_secs_f32();
    if hex::encode(hasher.finalize()) == SPEEDTEST_FILE_CHECKSUM {
        return Ok(result_time);
    }

    Err(anyhow!(
        "Failed to download test data from {}, please check your network connection!",
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
    for recipe in recipes
        .variants
        .into_iter()
        .filter(|x| x.retro == IS_RETRO && !x.tarballs.is_empty() && x.name != "BuildKit")
    {
        let mut sorted_tarballs: Vec<Tarball> = recipe
            .tarballs
            .into_iter()
            .filter(|x| x.arch == arch_name)
            .collect();
        sorted_tarballs.sort_by(|a, b| b.date.cmp(&a.date));
        let candidate = sorted_tarballs.first().unwrap();
        results.push(VariantEntry {
            name: recipe.name.clone(),
            size: candidate.download_size as u64,
            install_size: candidate.inst_size as u64,
            date: candidate.date.clone(),
            url: candidate.path.clone(),
            sha256sum: candidate.sha256sum.clone(),
        });
    }
    results.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(results)
}

#[test]
fn test_speedtest_mirrors() {
    let manifest = fetch_recipe().map_err(|e| e.to_string()).unwrap();
    let mirrors = fetch_mirrors(&manifest);
    let new_mirrors = speedtest_mirrors(mirrors);
    dbg!(new_mirrors);
}
