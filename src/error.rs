use thiserror::Error;

#[derive(Error, Debug)]
pub enum SkmError {
    #[error("Invalid GitHub URL: {0}")]
    InvalidUrl(String),
    #[error("SKILL.md not found in the specified directory")]
    SkillMdNotFound,
    #[error("GitHub API error: {0}")]
    GitHubApi(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
