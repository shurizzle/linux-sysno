#![allow(dead_code)]

use std::{fmt, io::Read};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};

use serde::Deserialize;
use ureq::Response;

#[derive(Deserialize)]
struct Release {
    pub version: String,
    pub moniker: String,
}

#[derive(Deserialize)]
struct Releases {
    pub releases: Vec<Release>,
}

pub fn latest_version() -> Result<Box<str>> {
    let releases: Releases = serde_json::from_reader(
        ureq::get("https://www.kernel.org/releases.json")
            .call()
            .wrap_err("Failed to fetch releases.json")?
            .into_reader(),
    )
    .wrap_err("Failed to parse releases.json")?;

    for release in releases.releases {
        if release.moniker == "mainline" {
            return Ok(release.version.into_boxed_str());
        }
    }
    bail!("mainline kernel version not found")
}

pub fn fetch_file<S1: fmt::Display, S2: fmt::Display>(
    version: S1,
    file: S2,
) -> Result<Box<dyn Read + Send + Sync>> {
    let url = format!("https://raw.githubusercontent.com/torvalds/linux/v{version}/{file}");

    ureq::get(&url)
        .call()
        .wrap_err("Failed to fetch kernel file")
        .map(Response::into_reader)
}
