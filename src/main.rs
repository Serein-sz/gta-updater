mod conf;

use anyhow::{Result, anyhow};
use serde::Deserialize;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[cfg(target_os = "linux")]
const OS: &str = "linux";

#[cfg(target_os = "windows")]
const OS: &str = "windows";

#[cfg(target_os = "macos")]
const OS: &str = "darwin";

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "amd64";

#[cfg(target_arch = "aarch64")]
const ARCH: &str = "arm64";

async fn download_file(url: &str, save_path: &str) -> Result<()> {
    let client = reqwest::Client::new();

    let mut resp = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "cpa-updater")
        .send()
        .await?
        .error_for_status()?;

    let mut file = tokio::fs::File::create(save_path).await?;

    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conf = conf::AppConfig::load()?;
    dbg!(&conf);
    let client = reqwest::Client::new();
    for app in conf.apps.iter() {
        let release: Release = client
            .get(format!(
                "https://api.github.com/repos/{}/{}/releases/latest",
                conf.github_owner, &app.name
            ))
            .header(reqwest::header::USER_AGENT, &app.name)
            .header(reqwest::header::ACCEPT, "application/vnd.github+json")
            .send()
            .await?
            .json()
            .await?;

        println!("tag: {}", release.tag_name);
        let r = release
            .assets
            .iter()
            .find(|&asset| asset.name.contains(&format!("{OS}-{ARCH}")));
        if let Some(asset) = r {
            dbg!(asset);
        }
    }
    Ok(())
}
