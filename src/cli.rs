use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "skm", about = "Skill Manager for Claude Code")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Install a skill from GitHub
    #[command(alias = "i")]
    Install {
        /// GitHub URL of the skill directory
        url: String,
        /// Install to ~/.claude/skills/ (global)
        #[arg(long, short)]
        global: bool,
    },
    /// Uninstall a skill
    Uninstall {
        /// Skill name to uninstall
        name: String,
        /// Uninstall from ~/.claude/skills/ (global)
        #[arg(long, short)]
        global: bool,
    },
    /// List installed skills
    List {
        /// List skills from ~/.claude/skills/ (global)
        #[arg(long, short)]
        global: bool,
    },
}
