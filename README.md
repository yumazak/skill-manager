# skm - Skill Manager

Download Claude Code skills from GitHub.

## Install

```bash
mise use -g github:yumazak/skill-manager@latest
```

## Usage

```bash
# Download to ~/.claude/skills/
skm https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy

# Download to ./.claude/skills/ (local project)
skm https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy --local
```

## Requirements

- Target directory must contain `SKILL.md` (validates it's a valid skill)
- Public GitHub repositories only (no authentication required)
