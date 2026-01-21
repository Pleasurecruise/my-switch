import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Eye, EyeOff } from "@my-monorepo/ui/icons";
import { Field, FieldLabel } from "@my-monorepo/ui/components/field";
import { Input } from "@my-monorepo/ui/components/input";
import { Button } from "@my-monorepo/ui/components/button";

interface EnvConfig {
	cs_base_url: string;
	cs_auth_token: string;
}

interface CsConfigGroup {
	base_url: string;
	auth_token: string;
	active: boolean;
}

interface AnthropicConfigGroup {
	base_url: string;
	auth_token: string;
	active: boolean;
}

interface AnthropicConfig {
	base_url: string;
	auth_token: string;
}

interface CodexConfig {
	base_url: string;
	api_key: string;
}

function App() {
	const [baseUrl, setBaseUrl] = useState("");
	const [authToken, setAuthToken] = useState("");
	const [showToken, setShowToken] = useState(false);
	const [anthropicBaseUrl, setAnthropicBaseUrl] = useState("");
	const [anthropicAuthToken, setAnthropicAuthToken] = useState("");
	const [showAnthropicToken, setShowAnthropicToken] = useState(false);
	const [codexBaseUrl, setCodexBaseUrl] = useState("");
	const [codexApiKey, setCodexApiKey] = useState("");
	const [showCodexKey, setShowCodexKey] = useState(false);
	const [loading, setLoading] = useState(true);
	const [saving, setSaving] = useState(false);
	const [message, setMessage] = useState("");
	const [csConfigGroups, setCsConfigGroups] = useState<CsConfigGroup[]>([]);
	const [anthropicConfigGroups, setAnthropicConfigGroups] = useState<
		AnthropicConfigGroup[]
	>([]);
	const [droidConfig, setDroidConfig] = useState<CodexConfig | null>(null);
	const [opencodeConfig, setOpencodeConfig] = useState<CodexConfig | null>(
		null,
	);

	function showMessage(msg: string, autoClear = true) {
		setMessage(msg);
		if (autoClear && !msg.includes("Failed")) {
			setTimeout(() => setMessage(""), 2000);
		}
	}

	useEffect(() => {
		loadConfig();
	}, []);

	async function loadConfig() {
		try {
			const [
				envConfig,
				codexConfig,
				configGroups,
				anthropicGroups,
				anthropicConfig,
				droid,
				opencode,
			] = await Promise.all([
				invoke<EnvConfig>("read_env_config"),
				invoke<CodexConfig>("read_codex_config"),
				invoke<CsConfigGroup[]>("read_cs_config_groups"),
				invoke<AnthropicConfigGroup[]>("read_anthropic_config_groups"),
				invoke<AnthropicConfig>("read_anthropic_config"),
				invoke<CodexConfig>("read_droid_config").catch(() => null),
				invoke<CodexConfig>("read_opencode_config").catch(() => null),
			]);
			setBaseUrl(envConfig.cs_base_url);
			setAuthToken(envConfig.cs_auth_token);
			setCodexBaseUrl(codexConfig.base_url);
			setCodexApiKey(codexConfig.api_key);
			setCsConfigGroups(configGroups);
			setAnthropicConfigGroups(anthropicGroups);
			setAnthropicBaseUrl(anthropicConfig.base_url);
			setAnthropicAuthToken(anthropicConfig.auth_token);
			setDroidConfig(droid);
			setOpencodeConfig(opencode);
		} catch (error) {
			setMessage(`Failed to load config: ${error}`);
		} finally {
			setLoading(false);
		}
	}

	async function switchConfig(index: number) {
		const group = csConfigGroups[index];
		if (group) {
			setBaseUrl(group.base_url);
			setAuthToken(group.auth_token);
		}
	}

	async function switchAnthropicConfig(index: number) {
		const group = anthropicConfigGroups[index];
		if (group) {
			setAnthropicBaseUrl(group.base_url);
			setAnthropicAuthToken(group.auth_token);
		}
	}

	async function saveConfig() {
		setSaving(true);
		setMessage("");
		try {
			await Promise.all([
				invoke("save_env_config", {
					config: {
						cs_base_url: baseUrl,
						cs_auth_token: authToken,
					},
				}),
				invoke("save_anthropic_config", {
					config: {
						base_url: anthropicBaseUrl,
						auth_token: anthropicAuthToken,
					},
				}),
				invoke("save_codex_config", {
					config: {
						base_url: codexBaseUrl,
						api_key: codexApiKey,
					},
				}),
			]);
			showMessage("Saved!");
		} catch (error) {
			showMessage(`Failed to save: ${error}`, false);
		} finally {
			setSaving(false);
		}
	}

	async function applyToDroid() {
		try {
			await invoke("apply_codex_to_droid", {
				config: {
					base_url: codexBaseUrl,
					api_key: codexApiKey,
				},
			});
			setDroidConfig({ base_url: codexBaseUrl, api_key: codexApiKey });
			showMessage("Applied to Droid!");
		} catch (error) {
			showMessage(`Failed to apply to Droid: ${error}`, false);
		}
	}

	async function applyToOpenCode() {
		try {
			await invoke("apply_codex_to_opencode", {
				config: {
					base_url: codexBaseUrl,
					api_key: codexApiKey,
				},
			});
			setOpencodeConfig({ base_url: codexBaseUrl, api_key: codexApiKey });
			showMessage("Applied to OpenCode!");
		} catch (error) {
			showMessage(`Failed to apply to OpenCode: ${error}`, false);
		}
	}

	const isDroidSynced =
		droidConfig?.base_url === codexBaseUrl &&
		droidConfig?.api_key === codexApiKey;
	const isOpencodeSynced =
		opencodeConfig?.base_url === codexBaseUrl &&
		opencodeConfig?.api_key === codexApiKey;

	if (loading) {
		return (
			<main className="flex h-screen items-center justify-center">
				<p>Loading...</p>
			</main>
		);
	}

	return (
		<main className="flex h-screen flex-col justify-center p-6 gap-6">
			<h1 className="text-xl font-bold text-center">
				Environment Configuration
			</h1>

			<div className="grid grid-cols-3 gap-6">
				{/* Claude Code */}
				<section className="space-y-3">
					<div className="flex items-center justify-between">
						<h2 className="font-semibold">Claude Code</h2>
						{anthropicConfigGroups.length > 1 && (
							<div className="flex gap-1">
								{anthropicConfigGroups.map((group, index) => {
									const label = group.base_url.includes("anti")
										? "Gemini"
										: "GLM";
									const isSelected = group.base_url === anthropicBaseUrl;
									return (
										<Button
											key={index}
											variant={isSelected ? "default" : "outline"}
											size="sm"
											className="h-6 px-2 text-xs"
											onClick={() => switchAnthropicConfig(index)}
										>
											{label}
										</Button>
									);
								})}
							</div>
						)}
					</div>
					<Field>
						<FieldLabel className="text-xs">BASE_URL</FieldLabel>
						<Input
							type="text"
							placeholder="https://..."
							value={anthropicBaseUrl}
							onChange={(e) => setAnthropicBaseUrl(e.target.value)}
							className="h-9"
						/>
					</Field>
					<Field>
						<FieldLabel className="text-xs">AUTH_TOKEN</FieldLabel>
						<div className="relative">
							<Input
								type={showAnthropicToken ? "text" : "password"}
								placeholder="sk-..."
								value={anthropicAuthToken}
								onChange={(e) => setAnthropicAuthToken(e.target.value)}
								className="h-9 pr-9"
							/>
							<button
								type="button"
								onClick={() => setShowAnthropicToken(!showAnthropicToken)}
								className="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
							>
								{showAnthropicToken ? (
									<EyeOff className="size-4" />
								) : (
									<Eye className="size-4" />
								)}
							</button>
						</div>
					</Field>
				</section>

				{/* cc4cs */}
				<section className="space-y-3">
					<div className="flex items-center justify-between">
						<h2 className="font-semibold">cc4cs</h2>
						{csConfigGroups.length > 1 && (
							<div className="flex gap-1">
								{csConfigGroups.map((group, index) => {
									const label = group.base_url.includes("gemini")
										? "NEW"
										: "OLD";
									const isSelected = group.base_url === baseUrl;
									return (
										<Button
											key={index}
											variant={isSelected ? "default" : "outline"}
											size="sm"
											className="h-6 px-2 text-xs"
											onClick={() => switchConfig(index)}
										>
											{label}
										</Button>
									);
								})}
							</div>
						)}
					</div>
					<Field>
						<FieldLabel className="text-xs">BASE_URL</FieldLabel>
						<Input
							type="text"
							placeholder="https://..."
							value={baseUrl}
							onChange={(e) => setBaseUrl(e.target.value)}
							className="h-9"
						/>
					</Field>
					<Field>
						<FieldLabel className="text-xs">AUTH_TOKEN</FieldLabel>
						<div className="relative">
							<Input
								type={showToken ? "text" : "password"}
								placeholder="sk-..."
								value={authToken}
								onChange={(e) => setAuthToken(e.target.value)}
								className="h-9 pr-9"
							/>
							<button
								type="button"
								onClick={() => setShowToken(!showToken)}
								className="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
							>
								{showToken ? (
									<EyeOff className="size-4" />
								) : (
									<Eye className="size-4" />
								)}
							</button>
						</div>
					</Field>
				</section>

				{/* Codex */}
				<section className="space-y-3">
					<div className="flex items-center justify-between">
						<h2 className="font-semibold">Codex</h2>
					</div>
					<Field>
						<FieldLabel className="text-xs">BASE_URL</FieldLabel>
						<Input
							type="text"
							placeholder="https://..."
							value={codexBaseUrl}
							onChange={(e) => setCodexBaseUrl(e.target.value)}
							className="h-9"
						/>
					</Field>
					<Field>
						<FieldLabel className="text-xs">API_KEY</FieldLabel>
						<div className="relative">
							<Input
								type={showCodexKey ? "text" : "password"}
								placeholder="sk-..."
								value={codexApiKey}
								onChange={(e) => setCodexApiKey(e.target.value)}
								className="h-9 pr-9"
							/>
							<button
								type="button"
								onClick={() => setShowCodexKey(!showCodexKey)}
								className="absolute right-2.5 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
							>
								{showCodexKey ? (
									<EyeOff className="size-4" />
								) : (
									<Eye className="size-4" />
								)}
							</button>
						</div>
					</Field>
				</section>
			</div>

			<div className="flex flex-col items-center gap-2">
				<div className="flex items-center justify-center gap-3">
					<Button onClick={saveConfig} disabled={saving}>
						{saving ? "Saving..." : "Save"}
					</Button>
					<Button
						variant="outline"
						onClick={applyToDroid}
						disabled={isDroidSynced}
					>
						Codex → Droid
					</Button>
					<Button
						variant="outline"
						onClick={applyToOpenCode}
						disabled={isOpencodeSynced}
					>
						Codex → OpenCode
					</Button>
				</div>
				<p
					className={`text-sm h-5 ${message.includes("Failed") ? "text-red-500" : "text-green-500"}`}
				>
					{message}
				</p>
			</div>
		</main>
	);
}

export default App;
