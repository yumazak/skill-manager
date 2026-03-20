use crate::error::SkmError;
use serde::Deserialize;

pub struct GitHubUrl {
    pub owner: String,
    pub repo: String,
    pub branch: String,
    pub path: String,
    pub skill_name: String,
}

impl GitHubUrl {
    pub fn parse(url: &str) -> Result<Self, SkmError> {
        let stripped = url.strip_prefix("https://github.com/").ok_or_else(|| {
            SkmError::InvalidUrl("URL must start with https://github.com/".into())
        })?;

        let parts: Vec<&str> = stripped.splitn(5, '/').collect();
        if parts.len() < 5 || parts[2] != "tree" {
            return Err(SkmError::InvalidUrl(
                "Expected format: https://github.com/{owner}/{repo}/tree/{branch}/{path}".into(),
            ));
        }

        let path = parts[4].to_string();
        let skill_name = path.rsplit('/').next().unwrap_or(&path).to_string();

        Ok(Self {
            owner: parts[0].to_string(),
            repo: parts[1].to_string(),
            branch: parts[3].to_string(),
            path,
            skill_name,
        })
    }

    pub fn api_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/contents/{}?ref={}",
            self.owner, self.repo, self.path, self.branch
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct GitHubContent {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub download_url: Option<String>,
}

pub struct GitHubClient {
    client: reqwest::blocking::Client,
}

impl GitHubClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::builder()
                .user_agent("skm")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub fn fetch_contents(&self, api_url: &str) -> Result<Vec<GitHubContent>, SkmError> {
        let response = self.client.get(api_url).send()?;

        if !response.status().is_success() {
            return Err(SkmError::GitHubApi(format!(
                "HTTP {}: {}",
                response.status(),
                api_url
            )));
        }

        let contents: Vec<GitHubContent> = response.json()?;
        Ok(contents)
    }

    pub fn download_file(&self, url: &str) -> Result<Vec<u8>, SkmError> {
        let response = self.client.get(url).send()?;
        if !response.status().is_success() {
            return Err(SkmError::GitHubApi(format!(
                "HTTP {}: {}",
                response.status(),
                url
            )));
        }
        Ok(response.bytes()?.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_url() {
        let url =
            "https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy";
        let parsed = GitHubUrl::parse(url).unwrap();
        assert_eq!(parsed.owner, "NVIDIA");
        assert_eq!(parsed.repo, "OpenShell");
        assert_eq!(parsed.branch, "main");
        assert_eq!(parsed.path, ".agents/skills/generate-sandbox-policy");
        assert_eq!(parsed.skill_name, "generate-sandbox-policy");
    }

    #[test]
    fn parse_valid_url_with_nested_path() {
        let url = "https://github.com/owner/repo/tree/dev/path/to/skill";
        let parsed = GitHubUrl::parse(url).unwrap();
        assert_eq!(parsed.owner, "owner");
        assert_eq!(parsed.repo, "repo");
        assert_eq!(parsed.branch, "dev");
        assert_eq!(parsed.path, "path/to/skill");
        assert_eq!(parsed.skill_name, "skill");
    }

    #[test]
    fn parse_invalid_url_not_github() {
        let result = GitHubUrl::parse("https://gitlab.com/owner/repo/tree/main/path");
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_url_no_tree() {
        let result = GitHubUrl::parse("https://github.com/owner/repo/blob/main/file.rs");
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_url_too_short() {
        let result = GitHubUrl::parse("https://github.com/owner/repo");
        assert!(result.is_err());
    }

    #[test]
    fn api_url_format() {
        let url =
            "https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy";
        let parsed = GitHubUrl::parse(url).unwrap();
        assert_eq!(
            parsed.api_url(),
            "https://api.github.com/repos/NVIDIA/OpenShell/contents/.agents/skills/generate-sandbox-policy?ref=main"
        );
    }
}
