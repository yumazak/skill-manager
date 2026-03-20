# skm - Skill Manager

Download and manage Claude Code skills from GitHub.

## Install

```bash
mise use -g github:yumazak/skill-manager@latest
```

## Usage

```bash
# Install a skill (default: ./.claude/skills/)
skm install https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy
skm i https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy

# Install globally (~/.claude/skills/)
skm install -g https://github.com/NVIDIA/OpenShell/tree/main/.agents/skills/generate-sandbox-policy

# Uninstall a skill
skm uninstall generate-sandbox-policy
skm uninstall -g generate-sandbox-policy

# List installed skills
skm list
skm list -g
```

## Notes

- Target directory must contain `SKILL.md` (validates it's a valid skill)
- Public GitHub repositories only (no authentication required)
