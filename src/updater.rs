use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::path::Path;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[cfg(target_os = "linux")]
pub const OS: &str = "linux";

#[cfg(target_os = "windows")]
pub const OS: &str = "windows";

#[cfg(target_os = "macos")]
pub const OS: &str = "darwin";

#[cfg(target_arch = "x86_64")]
pub const ARCH: &str = "amd64";

#[cfg(target_arch = "aarch64")]
pub const ARCH: &str = "arm64";

pub async fn fetch_latest_release(
    client: &reqwest::Client,
    owner: &str,
    repo: &str,
    token: Option<&str>,
) -> Result<Release> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    let mut request = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, repo)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json");

    // Add Authorization header if token is provided
    if let Some(token) = token {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token));
    }

    let release = request
        .send()
        .await
        .context(format!("Failed to fetch release info for {}", repo))?
        .error_for_status()
        .context(format!("API request failed for {}", repo))?
        .json::<Release>()
        .await
        .context("Failed to parse release JSON")?;

    Ok(release)
}

pub fn find_matching_asset(assets: &[Asset]) -> Option<&Asset> {
    assets
        .iter()
        .find(|asset| asset.name.contains(&format!("{}-{}", OS, ARCH)))
}

pub async fn download_file(
    name: &str,
    url: &str,
    path: impl AsRef<Path>,
    token: Option<&str>,
) -> Result<()> {
    let client = reqwest::Client::new();
    let mut request = client.get(url);
    if let Some(token) = token {
        request = request.header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token));
    }
    let response = request
        .send()
        .await
        .context("Failed to start download")?
        .error_for_status()
        .context("Download request failed")?;

    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_message(format!("Downloading {}", name));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .context("Failed to create progress bar style")?
        .progress_chars("#>-"),
    );

    let mut file = tokio::fs::File::create(&path)
        .await
        .context(format!("Failed to create file at {:?}", path.as_ref()))?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Failed to read chunk")?;
        file.write_all(&chunk)
            .await
            .context("Failed to write to file")?;
        pb.inc(chunk.len() as u64);
    }

    pb.finish_with_message(format!("✓ Downloaded {}", name));
    #[cfg(unix)]
    {
        make_executable(path.as_ref())?;
    }
    Ok(())
}
#[cfg(unix)]
pub fn make_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .context("Failed to read file metadata")?
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).context("Failed to set executable permissions")?;
    Ok(())
}

pub fn compare_versions(current: &str, latest: &str) -> Result<std::cmp::Ordering> {
    let current_version = semver::Version::parse(current.trim_start_matches('v'))
        .context(format!("Invalid current version: {}", current))?;
    let latest_version = semver::Version::parse(latest.trim_start_matches('v'))
        .context(format!("Invalid latest version: {}", latest))?;

    Ok(current_version.cmp(&latest_version))
}
