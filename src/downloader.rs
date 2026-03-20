use crate::error::SkmError;
use crate::github::{GitHubClient, GitHubContent, GitHubUrl};
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_skills_dir(local: bool) -> Result<PathBuf, SkmError> {
    if local {
        return Ok(PathBuf::from(".claude").join("skills"));
    }

    let home = dirs::home_dir().ok_or_else(|| {
        SkmError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    })?;
    Ok(home.join(".claude").join("skills"))
}

pub fn resolve_destination(skill_name: &str, local: bool) -> Result<PathBuf, SkmError> {
    Ok(resolve_skills_dir(local)?.join(skill_name))
}

pub fn download_skill(github_url: &GitHubUrl, dest: &Path) -> Result<(), SkmError> {
    let client = GitHubClient::new();

    let contents = client.fetch_contents(&github_url.api_url())?;

    let has_skill_md = contents
        .iter()
        .any(|c| c.name == "SKILL.md" && c.content_type == "file");
    if !has_skill_md {
        return Err(SkmError::SkillMdNotFound);
    }

    fs::create_dir_all(dest)?;

    download_contents(&client, &contents, dest, github_url)?;

    Ok(())
}

fn download_contents(
    client: &GitHubClient,
    contents: &[GitHubContent],
    dest: &Path,
    github_url: &GitHubUrl,
) -> Result<(), SkmError> {
    for item in contents {
        let target = dest.join(&item.name);

        match item.content_type.as_str() {
            "file" => {
                if let Some(ref url) = item.download_url {
                    let data = client.download_file(url)?;
                    fs::write(&target, &data)?;
                    eprintln!("  Downloaded: {}", item.name);
                }
            }
            "dir" => {
                fs::create_dir_all(&target)?;
                let sub_api_url = format!(
                    "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
                    github_url.owner, github_url.repo, item.path, github_url.branch
                );
                let sub_contents = client.fetch_contents(&sub_api_url)?;
                download_contents(client, &sub_contents, &target, github_url)?;
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_destination_local() {
        let dest = resolve_destination("my-skill", true).unwrap();
        assert_eq!(dest, PathBuf::from(".claude/skills/my-skill"));
    }

    #[test]
    fn resolve_destination_global() {
        let dest = resolve_destination("my-skill", false).unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(dest, home.join(".claude/skills/my-skill"));
    }
}
