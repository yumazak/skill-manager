mod cli;
mod downloader;
mod error;
mod github;

use clap::Parser;
use cli::{Cli, Command};
use github::GitHubUrl;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Install { url, global } => {
            let github_url = GitHubUrl::parse(&url)?;
            let local = !global;
            let dest = downloader::resolve_destination(&github_url.skill_name, local)?;

            eprintln!(
                "Downloading skill '{}' from {}/{}...",
                github_url.skill_name, github_url.owner, github_url.repo
            );

            downloader::download_skill(&github_url, &dest)?;

            eprintln!(
                "Skill '{}' installed to {}",
                github_url.skill_name,
                dest.display()
            );
        }
        Command::Uninstall { name, global } => {
            let local = !global;
            let dest = downloader::resolve_destination(&name, local)?;

            if !dest.exists() {
                anyhow::bail!("Skill '{}' not found at {}", name, dest.display());
            }

            std::fs::remove_dir_all(&dest)?;
            eprintln!("Skill '{}' uninstalled from {}", name, dest.display());
        }
        Command::List { global } => {
            let local = !global;
            let skills_dir = downloader::resolve_skills_dir(local)?;

            if !skills_dir.exists() {
                return Ok(());
            }

            let mut entries: Vec<_> = std::fs::read_dir(&skills_dir)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect();
            entries.sort();

            for name in entries {
                println!("{name}");
            }
        }
    }

    Ok(())
}
