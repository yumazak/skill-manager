use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "skm", about = "Download Claude Code skills from GitHub")]
pub struct Cli {
    /// GitHub URL of the skill directory
    pub url: String,
    /// Download to ~/.claude/skills/ instead of ./.claude/skills/
    #[arg(long, short)]
    pub global: bool,
}
