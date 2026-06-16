mod cli;
mod conf;
mod updater;

use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let mut config = conf::AppConfig::load()
        .context("Failed to load configuration. Please ensure config.toml exists.")?;

    if args.verbose {
        println!("{}", "Configuration loaded:".bright_blue());
        println!("  Owner: {}", config.github_owner);
        println!("  Global path: {}", config.global_path);
        println!("  Apps: {}", config.apps.len());
    }

    let client = reqwest::Client::new();
    let mut updated_count = 0;

    for app in config.apps.iter_mut() {
        // Skip if specific app requested and this isn't it
        if let Some(ref target_app) = args.app {
            if app.name != *target_app && app.alias.as_ref() != Some(target_app) {
                continue;
            }
        }

        println!("\n{} {}", "Checking".bright_cyan(), app.name.bold());

        let release =
            match updater::fetch_latest_release(&client, &config.github_owner, &app.name).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("  {} Failed to fetch release: {}", "✗".red(), e);
                    continue;
                }
            };

        if args.verbose {
            println!("  Current version: {}", app.version);
            println!("  Latest version: {}", release.tag_name);
        }

        // Compare versions
        let needs_update = match updater::compare_versions(&app.version, &release.tag_name) {
            Ok(std::cmp::Ordering::Less) => true,
            Ok(std::cmp::Ordering::Equal) => {
                if args.force {
                    println!(
                        "  {} Already on latest version, but forcing update",
                        "→".yellow()
                    );
                    true
                } else {
                    println!(
                        "  {} Already on latest version ({})",
                        "✓".green(),
                        app.version
                    );
                    false
                }
            }
            Ok(std::cmp::Ordering::Greater) => {
                println!(
                    "  {} Current version is newer than latest release",
                    "→".yellow()
                );
                false
            }
            Err(e) => {
                eprintln!("  {} Failed to compare versions: {}", "✗".red(), e);
                continue;
            }
        };

        if !needs_update {
            continue;
        }

        // Find matching asset
        let asset = match updater::find_matching_asset(&release.assets) {
            Some(a) => a,
            None => {
                eprintln!(
                    "  {} No matching asset found for {}-{}",
                    "✗".red(),
                    updater::OS,
                    updater::ARCH
                );
                if args.verbose {
                    println!("  Available assets:");
                    for asset in &release.assets {
                        println!("    - {}", asset.name);
                    }
                }
                continue;
            }
        };

        if args.dry_run {
            println!(
                "  {} Would update {} → {}",
                "→".yellow(),
                app.version,
                release.tag_name
            );
            println!("    Asset: {}", asset.name);
            continue;
        }

        // Determine download path
        let install_path = if let Some(ref custom_path) = app.path {
            PathBuf::from(custom_path)
        } else {
            PathBuf::from(&config.global_path)
        };

        let binary_name = app.alias.as_ref().unwrap_or(&app.name);

        #[cfg(windows)]
        let binary_name = format!("{}.exe", binary_name);

        let file_path = install_path.join(&binary_name);

        // Ensure install directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .context(format!("Failed to create directory {:?}", parent))?;
        }

        // Download
        println!(
            "  {} Updating {} → {}",
            "↓".bright_green(),
            app.version,
            release.tag_name
        );

        if let Err(e) =
            updater::download_file(&asset.name, &asset.browser_download_url, &file_path).await
        {
            eprintln!("  {} Download failed: {}", "✗".red(), e);
            continue;
        }

        // Update version in config
        app.version = release.tag_name.clone();
        updated_count += 1;

        println!("  {} Installed to {:?}", "✓".green(), file_path);
    }

    // Save updated config
    if updated_count > 0 && !args.dry_run {
        config
            .rewrite()
            .context("Failed to save updated configuration")?;
        println!(
            "\n{} Updated {} app(s)",
            "✓".green().bold(),
            updated_count.to_string().bold()
        );
    } else if args.dry_run {
        println!("\n{} Dry run complete (no changes made)", "→".yellow());
    } else {
        println!("\n{} All apps are up to date", "✓".green().bold());
    }

    Ok(())
}
