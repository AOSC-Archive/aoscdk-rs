use anyhow::{anyhow, Result};
use reqwest;
use serde::Deserialize;
use std::env::consts::ARCH;

const MANIFEST_URL: &str = "https://releases.aosc.io/manifest/recipe.json";
const IS_RETRO: bool = false;

// mirror manifests
#[derive(Deserialize, Clone, Debug)]
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

#[derive(Debug, Clone)]
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
        "powerpc64" => Some("ppc64"),
        _ => None,
    }
}

pub fn download_file(url: &str) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    let resp = resp.error_for_status()?;

    Ok(resp)
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
        .filter(|x| x.retro == IS_RETRO && !x.tarballs.is_empty())
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
