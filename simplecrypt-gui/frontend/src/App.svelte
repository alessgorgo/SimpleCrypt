<script>
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  let activeTab = $state("encrypt");
  let filePath = $state("");
  let password = $state("");
  let selectedAlgorithm = $state("aes-256-gcm");
  let mode = $state("file"); // "file" or "directory"
  let status = $state(null); // { type: "success" | "error" | "info", message: string }
  let loading = $state(false);
  let algorithms = $state([]);
  let config = $state(null);

  // Settings tab state
  let showSettings = $state(false);

  async function loadAlgorithms() {
    try {
      algorithms = await invoke("list_algorithms");
    } catch (e) {
      console.error("Failed to load algorithms:", e);
    }
  }

  async function loadConfig() {
    try {
      config = await invoke("get_config");
      if (config?.encryption?.algorithm) {
        selectedAlgorithm = config.encryption.algorithm;
      }
    } catch (e) {
      console.error("Failed to load config:", e);
    }
  }

  // Load on mount
  loadAlgorithms();
  loadConfig();

  async function browseFile() {
    try {
      const selected = await open({
        multiple: false,
        directory: mode === "directory",
      });
      if (selected) {
        filePath = selected;
      }
    } catch (e) {
      console.error("File dialog error:", e);
    }
  }

  async function handleEncrypt() {
    if (!filePath || !password) {
      status = { type: "error", message: "Please provide a file path and password." };
      return;
    }

    loading = true;
    status = { type: "info", message: "Encrypting..." };

    try {
      let result;
      if (mode === "directory") {
        result = await invoke("encrypt_directory", {
          path: filePath,
          password: password,
          algorithm: selectedAlgorithm,
        });
        status = {
          type: result.error_count === 0 ? "success" : "error",
          message: `Encrypted ${result.success_count} files, ${result.error_count} errors`,
        };
      } else {
        result = await invoke("encrypt_file", {
          path: filePath,
          password: password,
          algorithm: selectedAlgorithm,
        });
        status = { type: "success", message: result.message };
      }
    } catch (e) {
      status = { type: "error", message: String(e) };
    } finally {
      loading = false;
    }
  }

  async function handleDecrypt() {
    if (!filePath || !password) {
      status = { type: "error", message: "Please provide a file path and password." };
      return;
    }

    loading = true;
    status = { type: "info", message: "Decrypting..." };

    try {
      let result;
      if (mode === "directory") {
        result = await invoke("decrypt_directory", {
          path: filePath,
          password: password,
        });
        status = {
          type: result.error_count === 0 ? "success" : "error",
          message: `Decrypted ${result.success_count} files, ${result.error_count} errors`,
        };
      } else {
        result = await invoke("decrypt_file", {
          path: filePath,
          password: password,
        });
        status = { type: "success", message: result.message };
      }
    } catch (e) {
      status = { type: "error", message: String(e) };
    } finally {
      loading = false;
    }
  }

  async function saveSettings() {
    if (!config) return;
    config.encryption.algorithm = selectedAlgorithm;
    try {
      await invoke("save_config", { config });
      status = { type: "success", message: "Settings saved." };
    } catch (e) {
      status = { type: "error", message: "Failed to save settings: " + e };
    }
  }
</script>

<h1>SimpleCrypt</h1>
<p class="subtitle">Multi-algorithm file encryption</p>

<div class="tab-bar">
  <button class="tab" class:active={!showSettings} onclick={() => (showSettings = false)}>
    Encrypt / Decrypt
  </button>
  <button class="tab" class:active={showSettings} onclick={() => (showSettings = true)}>
    Settings
  </button>
</div>

{#if !showSettings}
  <div class="card">
    <div class="mode-toggle">
      <button class="btn btn-secondary" class:active={mode === "file"} onclick={() => (mode = "file")}>
        Single File
      </button>
      <button class="btn btn-secondary" class:active={mode === "directory"} onclick={() => (mode = "directory")}>
        Directory
      </button>
    </div>

    <div class="form-group">
      <label>{mode === "file" ? "File Path" : "Directory Path"}</label>
      <div class="file-picker">
        <input type="text" bind:value={filePath} placeholder={mode === "file" ? "/path/to/file.txt" : "/path/to/directory"} />
        <button class="btn btn-secondary" onclick={browseFile}>Browse</button>
      </div>
    </div>

    <div class="form-group">
      <label>Password</label>
      <input type="password" bind:value={password} placeholder="Enter encryption password" />
    </div>

    <div class="form-group">
      <label>Algorithm</label>
      <select bind:value={selectedAlgorithm}>
        {#each algorithms as algo}
          <option value={algo.id}>
            {algo.display_name} {algo.is_aead ? "(AEAD)" : ""}
          </option>
        {/each}
      </select>
    </div>

    <div class="actions">
      <button class="btn btn-primary" onclick={handleEncrypt} disabled={loading}>
        {loading && activeTab === "encrypt" ? "Encrypting..." : "Encrypt"}
      </button>
      <button class="btn btn-primary" onclick={handleDecrypt} disabled={loading}>
        {loading && activeTab === "decrypt" ? "Decrypting..." : "Decrypt"}
      </button>
    </div>
  </div>

{:else}
  <div class="card">
    <div class="settings-row">
      <label>Default Algorithm</label>
      <select bind:value={selectedAlgorithm}>
        {#each algorithms as algo}
          <option value={algo.id}>{algo.display_name}</option>
        {/each}
      </select>
    </div>

    <div class="settings-row">
      <label>Theme</label>
      <select bind:value={config.ui.theme}>
        <option value="system">System</option>
        <option value="light">Light</option>
        <option value="dark">Dark</option>
      </select>
    </div>

    <div class="settings-row">
      <label>Auto-backup before encryption</label>
      <select bind:value={config.security.backup_on_encrypt}>
        <option value={true}>Enabled</option>
        <option value={false}>Disabled</option>
      </select>
    </div>

    <div class="actions">
      <button class="btn btn-primary" onclick={saveSettings}>Save Settings</button>
    </div>
  </div>
{/if}

{#if status}
  <div class="status {status.type}">
    {status.message}
  </div>
{/if}
