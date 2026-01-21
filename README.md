# my-switch

A desktop app to switch API configurations for CLI coding tools.

## Supported Tools

| Tool        | Config Location                                |
|-------------|------------------------------------------------|
| Claude Code | `~/.zshrc_secrets` + `~/.claude/settings.json` |
| cc4cs       | `~/.zshrc_secrets` + `~/.claude/settings.json` |
| Codex       | `~/.codex/config.toml` + `~/.codex/auth.json`  |
| Droid       | `~/.factory/settings.json`                     |
| OpenCode    | `~/.config/opencode/opencode.json`             |

## Setup

Add your API configs to `~/.zshrc_secrets`. Each config group requires two consecutive lines (BASE_URL + AUTH_TOKEN).

**Multiple configs:** Comment out inactive ones with `#`. The app will toggle comments when switching.

```bash
# Claude Code (active)
export ANTHROPIC_BASE_URL="https://api.example.com"
export ANTHROPIC_AUTH_TOKEN="sk-xxx"

# Claude Code (inactive, commented)
#export ANTHROPIC_BASE_URL="https://api.another.com"
#export ANTHROPIC_AUTH_TOKEN="sk-yyy"

# cc4cs (active)
export CS_BASE_URL="https://cs.example.com"
export CS_AUTH_TOKEN="sk-xxx"

# cc4cs (inactive)
#export CS_BASE_URL="https://cs.another.com"
#export CS_AUTH_TOKEN="sk-yyy"
```

## Usage

1. Click switch buttons (e.g., Gemini / GLM) to toggle between config groups
2. Edit values directly if needed
3. Click **Save** to apply changes
4. **Codex → Droid** / **Codex → OpenCode**: Sync Codex config to other tools