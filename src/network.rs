use failure::{format_err, Error};
use reqwest;
use serde_derive::{Deserialize, Serialize};
use std::env::consts::ARCH;

use lazy_static::lazy_static;
use regex::Regex;
use serde_yaml;

const RECIPE_URL: &str =
    "https://cdn.jsdelivr.net/gh/AOSC-Dev/aosc-portal-kiss.github.io@9bb2d45/data/distro.yml";

const MIRRORS_URL: &str = "https://aosc.io/api/mirrors";
const REPO_URL: &str = "https://releases.aosc.io/";
const IS_RETRO: bool = false;

lazy_static! {
    static ref LINK_PATTERN: Regex =
        Regex::new(r#"="(aosc-os_[a-zA-Z%0-9]+_(\d{8})(?:_amd64)?\.tar\.(?:gz|xz))"#).unwrap();
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
#[derive(Deserialize, Debug)]
pub struct MirrorData {
    pub name: String,
    pub region: String,
    pub url: String,
    pub updated: i32,
}
#[derive(Deserialize, Debug)]
pub struct MirrorList {
    #[serde(rename = "ref")]
    reference: i32,
    pub mirrors: Vec<MirrorData>,
}

#[derive(Debug)]
pub struct VariantEntry {
    pub name: String,
    pub size: u64,
    pub date: String,
    pub url: String,
}

fn fetch_recipe_inner() -> Result<VariantData, Error> {
    let recipe = reqwest::blocking::get(RECIPE_URL)?.text()?;

    Ok(serde_yaml::from_str(&recipe)?)
}

pub fn fetch_mirrors() -> Result<MirrorList, Error> {
    Ok(reqwest::blocking::get(MIRRORS_URL)?.json()?)
}

pub fn fetch_links(url: &str) -> Result<Vec<(String, String)>, Error> {
    let content = reqwest::blocking::get(url)?.text()?;
    let captures = LINK_PATTERN.captures_iter(&content);
    let mut links: Vec<(String, String)> = Vec::new();
    for capture in captures {
        // capture group 1: full name; capture group 2: date
        if let Some(cap) = capture.get(1) {
            if let Some(cap_2) = capture.get(2) {
                links.push((cap.as_str().to_owned(), cap_2.as_str().to_owned()));
            }
        }
    }

    Ok(links)
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

pub fn fetch_recipe() -> Result<Vec<VariantEntry>, Error> {
    let recipes = fetch_recipe_inner()?;
    let distro_list;
    let mut results: Vec<VariantEntry> = Vec::new();
    if IS_RETRO {
        distro_list = recipes.retro;
    } else {
        distro_list = recipes.general;
    }
    for entry in distro_list.list {
        let downloads = entry.downloads;
        let arch_name = get_arch_name();
        if arch_name.is_none() {
            return Err(format_err!("Unsupported architecture."));
        }
        let arch_name = arch_name.unwrap();
        let link = downloads
            .into_iter()
            .find(|download| download.name.to_ascii_lowercase() == arch_name);
        if link.is_none() {
            return Err(format_err!("Download link not found."));
        }
        let page = link.unwrap();
        let mut links = fetch_links(&page.url)?;
        if links.is_empty() {
            return Err(format_err!("Failed to parse download page."));
        }
        links.sort_by(|a, b| b.1.cmp(&a.1));
        let candidate = links.first_mut().unwrap();
        results.push(VariantEntry {
            name: entry.name,
            date: candidate.1.clone(),
            url: format!("{}/{}", page.url, candidate.0),
            size: 0
        });
    }

    Ok(results)
}
