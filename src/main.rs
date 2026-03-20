mod cli;
mod downloader;
mod error;
mod github;

use clap::Parser;
use cli::Cli;
use github::GitHubUrl;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let github_url = GitHubUrl::parse(&args.url)?;

    eprintln!(
        "Downloading skill '{}' from {}/{}...",
        github_url.skill_name, github_url.owner, github_url.repo
    );

    let local = !args.global;
    let dest = downloader::resolve_destination(&github_url.skill_name, local)?;

    downloader::download_skill(&github_url, &dest)?;

    eprintln!(
        "Skill '{}' downloaded to {}",
        github_url.skill_name,
        dest.display()
    );

    Ok(())
}
