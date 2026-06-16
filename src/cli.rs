use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "gta",
    version,
    about = "GitHub Release Asset Updater - Keep your tools up to date",
    long_about = None
)]
pub struct Cli {
    /// Specific app to update (update all if not specified)
    #[arg(short, long)]
    pub app: Option<String>,

    /// Force update even if already on latest version
    #[arg(short, long)]
    pub force: bool,

    /// Dry run - check for updates without downloading
    #[arg(short = 'n', long)]
    pub dry_run: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
