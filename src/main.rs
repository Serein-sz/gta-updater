mod conf;

use std::path::{Path, PathBuf};
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut conf = conf::AppConfig::load()?;
    dbg!(&conf);
    let client = reqwest::Client::new();

    for app in conf.apps.iter_mut() {
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

        if semver::Version::parse(release.tag_name.trim_start_matches("v")).unwrap() <= semver::Version::parse(app.version.trim_start_matches("v")).unwrap() {
            continue;
        }
        app.version = release.tag_name;
        let release = release
            .assets
            .iter()
            .find(|&asset| asset.name.contains(&format!("{OS}-{ARCH}")));
        if let Some(asset) = release {
            let path = PathBuf::from(&conf.global_path);
            download(&asset.name, &asset.browser_download_url, path.join(app.alias.as_ref().unwrap_or(&app.name).as_str())).await?;
        }
        
    }

    conf.rewrite()?;
    
    Ok(())
}

async fn download(name: &str, url: &str, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);

    pb.set_message(format!("{} downloding", name));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {msg} [{bar:40.cyan/blue}] \
             {bytes}/{total_bytes} ({eta})",
        )?
    );

    let mut file = tokio::fs::File::create(&path).await?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk).await?;

        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message(format!("{} finished", name));
    make_executable(path.as_ref())?;
    Ok(())
}

pub fn make_executable(path: &Path) -> anyhow::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = std::fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(path, perms)?;
    }

    Ok(())
}