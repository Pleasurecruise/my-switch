use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toml::Value as TomlValue;

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub cs_base_url: String,
    pub cs_auth_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CsConfigGroup {
    pub base_url: String,
    pub auth_token: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AnthropicConfigGroup {
    pub base_url: String,
    pub auth_token: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub base_url: String,
    pub auth_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct CodexConfig {
    pub base_url: String,
    pub api_key: String,
}

fn get_secrets_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".zshrc_secrets")
}

fn get_claude_settings_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".claude")
        .join("settings.json")
}

fn get_codex_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".codex")
        .join("config.toml")
}

fn get_codex_auth_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".codex")
        .join("auth.json")
}

fn get_droid_settings_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".factory")
        .join("settings.json")
}

fn get_opencode_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config")
        .join("opencode")
        .join("opencode.json")
}

fn parse_env_value(content: &str, key: &str) -> String {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(&format!("export {}=", key)) {
            let value = line
                .strip_prefix(&format!("export {}=", key))
                .unwrap_or("");
            return value.trim_matches('"').to_string();
        }
    }
    String::new()
}

fn update_env_value(content: &str, key: &str, new_value: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut found = false;

    for line in lines.iter_mut() {
        if line.trim().starts_with(&format!("export {}=", key)) {
            *line = format!("export {}=\"{}\"", key, new_value);
            found = true;
            break;
        }
    }

    if !found {
        lines.push(format!("export {}=\"{}\"", key, new_value));
    }

    lines.join("\n")
}

fn update_claude_settings(config: &EnvConfig) -> Result<(), String> {
    let path = get_claude_settings_path();

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read claude settings: {}", e))?;

    let mut json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse claude settings: {}", e))?;

    if let Some(env) = json.get_mut("env") {
        if let Some(env_obj) = env.as_object_mut() {
            env_obj.insert("CS_BASE_URL".to_string(), Value::String(config.cs_base_url.clone()));
            env_obj.insert("CS_AUTH_TOKEN".to_string(), Value::String(config.cs_auth_token.clone()));
        }
    } else {
        if let Some(obj) = json.as_object_mut() {
            let mut env_obj = serde_json::Map::new();
            env_obj.insert("CS_BASE_URL".to_string(), Value::String(config.cs_base_url.clone()));
            env_obj.insert("CS_AUTH_TOKEN".to_string(), Value::String(config.cs_auth_token.clone()));
            obj.insert("env".to_string(), Value::Object(env_obj));
        }
    }

    let pretty_json = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Failed to serialize claude settings: {}", e))?;

    fs::write(&path, pretty_json)
        .map_err(|e| format!("Failed to write claude settings: {}", e))?;

    Ok(())
}

#[tauri::command]
fn read_env_config() -> Result<EnvConfig, String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    Ok(EnvConfig {
        cs_base_url: parse_env_value(&content, "CS_BASE_URL"),
        cs_auth_token: parse_env_value(&content, "CS_AUTH_TOKEN"),
    })
}

#[tauri::command]
fn read_cs_config_groups() -> Result<Vec<CsConfigGroup>, String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut groups: Vec<CsConfigGroup> = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check for active config: export CS_BASE_URL=
        if line.starts_with("export CS_BASE_URL=") {
            let base_url = line
                .strip_prefix("export CS_BASE_URL=")
                .unwrap_or("")
                .trim_matches('"')
                .to_string();

            // Look for the next line with CS_AUTH_TOKEN
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.starts_with("export CS_AUTH_TOKEN=") {
                    let auth_token = next_line
                        .strip_prefix("export CS_AUTH_TOKEN=")
                        .unwrap_or("")
                        .trim_matches('"')
                        .to_string();

                    groups.push(CsConfigGroup {
                        base_url,
                        auth_token,
                        active: true,
                    });
                    i += 2;
                    continue;
                }
            }
        }

        // Check for commented config: #export CS_BASE_URL=
        if line.starts_with("#export CS_BASE_URL=") {
            let base_url = line
                .strip_prefix("#export CS_BASE_URL=")
                .unwrap_or("")
                .trim_matches('"')
                .to_string();

            // Look for the next line with commented CS_AUTH_TOKEN
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.starts_with("#export CS_AUTH_TOKEN=") {
                    let auth_token = next_line
                        .strip_prefix("#export CS_AUTH_TOKEN=")
                        .unwrap_or("")
                        .trim_matches('"')
                        .to_string();

                    groups.push(CsConfigGroup {
                        base_url,
                        auth_token,
                        active: false,
                    });
                    i += 2;
                    continue;
                }
            }
        }

        i += 1;
    }

    Ok(groups)
}

#[tauri::command]
fn switch_cs_config(index: usize) -> Result<(), String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let groups = read_cs_config_groups()?;
    if index >= groups.len() {
        return Err("Invalid config index".to_string());
    }

    let target_group = &groups[index];
    if target_group.active {
        return Ok(()); // Already active
    }

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    for line in lines.iter_mut() {
        let trimmed = line.trim().to_string();

        // Comment out active config
        if trimmed.starts_with("export CS_BASE_URL=") || trimmed.starts_with("export CS_AUTH_TOKEN=") {
            *line = format!("#{}", trimmed);
            continue;
        }

        // Uncomment the target config
        if trimmed.starts_with("#export CS_BASE_URL=") {
            let url = trimmed
                .strip_prefix("#export CS_BASE_URL=")
                .unwrap_or("")
                .trim_matches('"');
            if url == target_group.base_url {
                *line = trimmed.strip_prefix("#").unwrap_or(&trimmed).to_string();
                continue;
            }
        }

        if trimmed.starts_with("#export CS_AUTH_TOKEN=") {
            let token = trimmed
                .strip_prefix("#export CS_AUTH_TOKEN=")
                .unwrap_or("")
                .trim_matches('"');
            if token == target_group.auth_token {
                *line = trimmed.strip_prefix("#").unwrap_or(&trimmed).to_string();
            }
        }
    }

    fs::write(&path, lines.join("\n"))
        .map_err(|e| format!("Failed to write file: {}", e))?;

    // Also update ~/.claude/settings.json
    let config = EnvConfig {
        cs_base_url: target_group.base_url.clone(),
        cs_auth_token: target_group.auth_token.clone(),
    };
    update_claude_settings(&config)?;

    Ok(())
}

#[tauri::command]
fn read_anthropic_config_groups() -> Result<Vec<AnthropicConfigGroup>, String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut groups: Vec<AnthropicConfigGroup> = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check for active config: export ANTHROPIC_BASE_URL=
        if line.starts_with("export ANTHROPIC_BASE_URL=") {
            let base_url = line
                .strip_prefix("export ANTHROPIC_BASE_URL=")
                .unwrap_or("")
                .trim_matches('"')
                .to_string();

            // Look for the next line with ANTHROPIC_AUTH_TOKEN
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.starts_with("export ANTHROPIC_AUTH_TOKEN=") {
                    let auth_token = next_line
                        .strip_prefix("export ANTHROPIC_AUTH_TOKEN=")
                        .unwrap_or("")
                        .trim_matches('"')
                        .to_string();

                    groups.push(AnthropicConfigGroup {
                        base_url,
                        auth_token,
                        active: true,
                    });
                    i += 2;
                    continue;
                }
            }
        }

        // Check for commented config: #export ANTHROPIC_BASE_URL=
        if line.starts_with("#export ANTHROPIC_BASE_URL=") {
            let base_url = line
                .strip_prefix("#export ANTHROPIC_BASE_URL=")
                .unwrap_or("")
                .trim_matches('"')
                .to_string();

            // Look for the next line with commented ANTHROPIC_AUTH_TOKEN
            if i + 1 < lines.len() {
                let next_line = lines[i + 1].trim();
                if next_line.starts_with("#export ANTHROPIC_AUTH_TOKEN=") {
                    let auth_token = next_line
                        .strip_prefix("#export ANTHROPIC_AUTH_TOKEN=")
                        .unwrap_or("")
                        .trim_matches('"')
                        .to_string();

                    groups.push(AnthropicConfigGroup {
                        base_url,
                        auth_token,
                        active: false,
                    });
                    i += 2;
                    continue;
                }
            }
        }

        i += 1;
    }

    Ok(groups)
}

#[tauri::command]
fn switch_anthropic_config(index: usize) -> Result<(), String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let groups = read_anthropic_config_groups()?;
    if index >= groups.len() {
        return Err("Invalid config index".to_string());
    }

    let target_group = &groups[index];
    if target_group.active {
        return Ok(()); // Already active
    }

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    for line in lines.iter_mut() {
        let trimmed = line.trim().to_string();

        // Comment out active config
        if trimmed.starts_with("export ANTHROPIC_BASE_URL=") || trimmed.starts_with("export ANTHROPIC_AUTH_TOKEN=") {
            *line = format!("#{}", trimmed);
            continue;
        }

        // Uncomment the target config
        if trimmed.starts_with("#export ANTHROPIC_BASE_URL=") {
            let url = trimmed
                .strip_prefix("#export ANTHROPIC_BASE_URL=")
                .unwrap_or("")
                .trim_matches('"');
            if url == target_group.base_url {
                *line = trimmed.strip_prefix("#").unwrap_or(&trimmed).to_string();
                continue;
            }
        }

        if trimmed.starts_with("#export ANTHROPIC_AUTH_TOKEN=") {
            let token = trimmed
                .strip_prefix("#export ANTHROPIC_AUTH_TOKEN=")
                .unwrap_or("")
                .trim_matches('"');
            if token == target_group.auth_token {
                *line = trimmed.strip_prefix("#").unwrap_or(&trimmed).to_string();
            }
        }
    }

    fs::write(&path, lines.join("\n"))
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[tauri::command]
fn save_env_config(config: EnvConfig) -> Result<(), String> {
    // Update ~/.zshrc_secrets
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut updated = update_env_value(&content, "CS_BASE_URL", &config.cs_base_url);
    updated = update_env_value(&updated, "CS_AUTH_TOKEN", &config.cs_auth_token);

    fs::write(&path, updated)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    // Update ~/.claude/settings.json
    update_claude_settings(&config)?;

    Ok(())
}

#[tauri::command]
fn read_anthropic_config() -> Result<AnthropicConfig, String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    Ok(AnthropicConfig {
        base_url: parse_env_value(&content, "ANTHROPIC_BASE_URL"),
        auth_token: parse_env_value(&content, "ANTHROPIC_AUTH_TOKEN"),
    })
}

#[tauri::command]
fn save_anthropic_config(config: AnthropicConfig) -> Result<(), String> {
    let path = get_secrets_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let mut updated = update_env_value(&content, "ANTHROPIC_BASE_URL", &config.base_url);
    updated = update_env_value(&updated, "ANTHROPIC_AUTH_TOKEN", &config.auth_token);

    fs::write(&path, updated)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[tauri::command]
fn read_codex_config() -> Result<CodexConfig, String> {
    // Read base_url from config.toml
    let config_path = get_codex_config_path();
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read codex config: {}", e))?;

    let toml: TomlValue = config_content.parse()
        .map_err(|e| format!("Failed to parse codex config: {}", e))?;

    let base_url = toml
        .get("model_providers")
        .and_then(|mp| mp.get("custom"))
        .and_then(|custom| custom.get("base_url"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Read api_key from auth.json
    let auth_path = get_codex_auth_path();
    let auth_content = fs::read_to_string(&auth_path)
        .map_err(|e| format!("Failed to read codex auth: {}", e))?;

    let auth_json: Value = serde_json::from_str(&auth_content)
        .map_err(|e| format!("Failed to parse codex auth: {}", e))?;

    let api_key = auth_json
        .get("OPENAI_API_KEY")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Ok(CodexConfig { base_url, api_key })
}

#[tauri::command]
fn save_codex_config(config: CodexConfig) -> Result<(), String> {
    // Update base_url in config.toml
    let config_path = get_codex_config_path();
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read codex config: {}", e))?;

    // Use regex-like replacement for TOML base_url under [model_providers.custom]
    let mut lines: Vec<String> = config_content.lines().map(|s| s.to_string()).collect();
    let mut in_custom_section = false;

    for line in lines.iter_mut() {
        let trimmed = line.trim();
        if trimmed == "[model_providers.custom]" {
            in_custom_section = true;
        } else if trimmed.starts_with('[') && in_custom_section {
            in_custom_section = false;
        } else if in_custom_section && trimmed.starts_with("base_url") {
            *line = format!("base_url = \"{}\"", config.base_url);
        }
    }

    fs::write(&config_path, lines.join("\n"))
        .map_err(|e| format!("Failed to write codex config: {}", e))?;

    // Update OPENAI_API_KEY in auth.json
    let auth_path = get_codex_auth_path();
    let auth_content = fs::read_to_string(&auth_path)
        .map_err(|e| format!("Failed to read codex auth: {}", e))?;

    let mut auth_json: Value = serde_json::from_str(&auth_content)
        .map_err(|e| format!("Failed to parse codex auth: {}", e))?;

    if let Some(obj) = auth_json.as_object_mut() {
        obj.insert("OPENAI_API_KEY".to_string(), Value::String(config.api_key));
    }

    let pretty_json = serde_json::to_string_pretty(&auth_json)
        .map_err(|e| format!("Failed to serialize codex auth: {}", e))?;

    fs::write(&auth_path, pretty_json)
        .map_err(|e| format!("Failed to write codex auth: {}", e))?;

    Ok(())
}

#[tauri::command]
fn read_droid_config() -> Result<CodexConfig, String> {
    let path = get_droid_settings_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read droid settings: {}", e))?;

    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse droid settings: {}", e))?;

    let base_url = json
        .get("customModels")
        .and_then(|cm| cm.as_array())
        .and_then(|arr| arr.get(0))
        .and_then(|m| m.get("baseUrl"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let api_key = json
        .get("customModels")
        .and_then(|cm| cm.as_array())
        .and_then(|arr| arr.get(0))
        .and_then(|m| m.get("apiKey"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Ok(CodexConfig { base_url, api_key })
}

#[tauri::command]
fn read_opencode_config() -> Result<CodexConfig, String> {
    let path = get_opencode_config_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read opencode config: {}", e))?;

    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse opencode config: {}", e))?;

    let base_url = json
        .get("provider")
        .and_then(|p| p.get("openai"))
        .and_then(|o| o.get("options"))
        .and_then(|opt| opt.get("baseURL"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let api_key = json
        .get("provider")
        .and_then(|p| p.get("openai"))
        .and_then(|o| o.get("options"))
        .and_then(|opt| opt.get("apiKey"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Ok(CodexConfig { base_url, api_key })
}

#[tauri::command]
fn apply_codex_to_droid(config: CodexConfig) -> Result<(), String> {
    let path = get_droid_settings_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read droid settings: {}", e))?;

    let mut json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse droid settings: {}", e))?;

    // Update customModels[0].baseUrl and customModels[0].apiKey
    if let Some(custom_models) = json.get_mut("customModels") {
        if let Some(models) = custom_models.as_array_mut() {
            if let Some(first_model) = models.get_mut(0) {
                if let Some(obj) = first_model.as_object_mut() {
                    obj.insert("baseUrl".to_string(), Value::String(config.base_url));
                    obj.insert("apiKey".to_string(), Value::String(config.api_key));
                }
            }
        }
    }

    let pretty_json = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Failed to serialize droid settings: {}", e))?;

    fs::write(&path, pretty_json)
        .map_err(|e| format!("Failed to write droid settings: {}", e))?;

    Ok(())
}

#[tauri::command]
fn apply_codex_to_opencode(config: CodexConfig) -> Result<(), String> {
    let path = get_opencode_config_path();
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read opencode config: {}", e))?;

    let mut json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse opencode config: {}", e))?;

    // Update provider.openai.options.baseURL and provider.openai.options.apiKey
    if let Some(provider) = json.get_mut("provider") {
        if let Some(openai) = provider.get_mut("openai") {
            if let Some(options) = openai.get_mut("options") {
                if let Some(obj) = options.as_object_mut() {
                    obj.insert("baseURL".to_string(), Value::String(config.base_url));
                    obj.insert("apiKey".to_string(), Value::String(config.api_key));
                }
            }
        }
    }

    let pretty_json = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Failed to serialize opencode config: {}", e))?;

    fs::write(&path, pretty_json)
        .map_err(|e| format!("Failed to write opencode config: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_env_config, save_env_config, read_codex_config, save_codex_config, read_cs_config_groups, switch_cs_config, read_anthropic_config_groups, switch_anthropic_config, read_anthropic_config, save_anthropic_config, read_droid_config, read_opencode_config, apply_codex_to_droid, apply_codex_to_opencode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
