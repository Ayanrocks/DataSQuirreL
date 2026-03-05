<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { invoke } from "@tauri-apps/api/core";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  import { save } from "@tauri-apps/plugin-dialog";
  import { notificationMsg } from "../stores";
  import {
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_ERROR,
  } from "../constants/constants";

  export let showModal = false;
  export let columns: string[] = [];
  export let data: any[] = [];
  export let onClose: () => void;

  interface ExportFormat {
    id: string;
    label: string;
    ext: string;
  }

  const exportFormats: ExportFormat[] = [
    { id: "csv", label: "CSV", ext: ".csv" },
    { id: "json", label: "JSON", ext: ".json" },
  ];

  let selectedFormatId = "csv";
  let includeHeaders = true;
  let exportPath = "";
  let baseExportDir = "";

  // Reactive default filename
  $: defaultFileName = `export${exportFormats.find((f) => f.id === selectedFormatId)?.ext || ".csv"}`;
  $: if (baseExportDir) {
    exportPath = `${baseExportDir}/${defaultFileName}`;
  }

  onMount(async () => {
    try {
      const res: any = await invoke("get_export_path");
      if (res.status === 200 && res.data) {
        baseExportDir = res.data;
        exportPath = `${baseExportDir}/${defaultFileName}`;
      } else {
        baseExportDir = "Downloads";
        exportPath = `${baseExportDir}/${defaultFileName}`;
      }
    } catch (e) {
      console.error("Failed to get export path", e);
      baseExportDir = "Downloads";
      exportPath = `${baseExportDir}/${defaultFileName}`;
    }
  });

  async function handleBrowsePath() {
    try {
      const activeFormat = exportFormats.find((f) => f.id === selectedFormatId);
      const extWithoutDot = activeFormat?.ext.replace(".", "") || "csv";

      const selectedPath = await save({
        defaultPath: exportPath,
        filters: [
          {
            name: activeFormat?.label || "Data",
            extensions: [extWithoutDot],
          },
        ],
      });

      if (selectedPath) {
        exportPath = selectedPath;
        // Optionally update the base dir in case they change formats later
        const separator = selectedPath.includes("\\") ? "\\" : "/";
        const parts = selectedPath.split(separator);
        parts.pop();
        baseExportDir = parts.join(separator);
      }
    } catch (e) {
      console.error("Failed to open save dialog:", e);
    }
  }

  const generateExportContent = (): string => {
    if (selectedFormatId === "csv") {
      let csvContent = "";
      if (includeHeaders) {
        csvContent +=
          columns.map((c) => `"${c.replace(/"/g, '""')}"`).join(",") + "\n";
      }
      for (const row of data) {
        const rowValues = columns.map((col) => {
          let val = row[col];
          if (val === null || val === undefined) val = "";
          val = String(val).replace(/"/g, '""');
          return `"${val}"`;
        });
        csvContent += rowValues.join(",") + "\n";
      }
      return csvContent;
    } else if (selectedFormatId === "json") {
      // JSON ignores includeHeaders as keys are always present
      return JSON.stringify(data, null, 2);
    }
    return "";
  };

  async function handleCopyToClipboard() {
    try {
      const content = generateExportContent();
      await navigator.clipboard.writeText(content);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_SUCCESS,
        message: "Export content copied to clipboard!",
      });
      onClose();
    } catch (e) {
      console.error(e);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "Failed to copy to clipboard.",
      });
    }
  }

  async function handleSaveToFile() {
    if (!exportPath) {
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "Please specify a valid export path.",
      });
      return;
    }

    try {
      const content = generateExportContent();
      await writeTextFile(exportPath, content);

      // Save the path for next time
      await invoke("save_export_path", { path: exportPath });

      notificationMsg.set({
        type: NOTIFICATION_TYPE_SUCCESS,
        message: `Successfully exported to ${exportPath}`,
      });
      onClose();
    } catch (e) {
      console.error(e);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: `Failed to save file: ${String(e)}`,
      });
    }
  }

  // Handle escape key
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && showModal) {
      onClose();
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });
  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

{#if showModal}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="modal-overlay"
    transition:fade={{ duration: 150 }}
    onclick={onClose}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>Export Data</h2>
        <button class="close-btn" onclick={onClose}>
          <i class="fa-solid fa-xmark"></i>
        </button>
      </div>

      <div class="modal-body">
        <p class="description">Export {data.length} currently visible rows.</p>

        <div class="form-group">
          <label for="export-format">Export Format</label>
          <select id="export-format" bind:value={selectedFormatId}>
            {#each exportFormats as format}
              <option value={format.id}>{format.label}</option>
            {/each}
          </select>
        </div>

        {#if selectedFormatId === "csv"}
          <div class="form-group checkbox-group">
            <input
              type="checkbox"
              id="include-headers"
              bind:checked={includeHeaders}
            />
            <label for="include-headers">Include Headers in export</label>
          </div>
        {/if}

        <div class="form-group">
          <label for="export-path">Save To File Path</label>
          <div class="input-with-button">
            <input
              type="text"
              id="export-path"
              bind:value={exportPath}
              placeholder="e.g. /home/user/Downloads/export.csv"
            />
            <button
              class="icon-btn-secondary"
              aria-label="Browse Path"
              onclick={handleBrowsePath}
            >
              <i class="fa-solid fa-folder-open"></i>
            </button>
          </div>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={handleCopyToClipboard}>
          <i class="fa-solid fa-clipboard"></i> Copy to Clipboard
        </button>
        <button class="btn btn-primary" onclick={handleSaveToFile}>
          <i class="fa-solid fa-file-export"></i> Save to File
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background-color: white;
    border-radius: 8px;
    width: 450px;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
    display: flex;
    flex-direction: column;
    color: #1f2937;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid #e5e7eb;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.25rem;
    font-weight: 600;
  }

  .close-btn {
    background: transparent;
    border: none;
    color: #6b7280;
    cursor: pointer;
    font-size: 1.25rem;
    padding: 4px;
  }

  .close-btn:hover {
    color: #111827;
  }

  .modal-body {
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .description {
    margin: 0;
    font-size: 0.875rem;
    color: #4b5563;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
  }

  .form-group input[type="text"],
  .form-group select {
    padding: 8px 12px;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 0.875rem;
    color: #111827;
    outline: none;
  }

  .form-group input[type="text"]:focus,
  .form-group select:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 1px #3b82f6;
  }

  .input-with-button {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .input-with-button input {
    flex: 1;
  }

  .icon-btn-secondary {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    background-color: #f3f4f6;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    color: #4b5563;
    cursor: pointer;
    transition: all 0.2s;
  }

  .icon-btn-secondary:hover {
    background-color: #e5e7eb;
    color: #111827;
  }

  .checkbox-group {
    flex-direction: row;
    align-items: center;
    gap: 8px;
  }

  .checkbox-group input[type="checkbox"] {
    margin: 0;
    cursor: pointer;
  }

  .checkbox-group label {
    cursor: pointer;
  }

  .modal-footer {
    padding: 16px 20px;
    border-top: 1px solid #e5e7eb;
    display: flex;
    justify-content: flex-end;
    gap: 12px;
  }

  .btn {
    padding: 8px 16px;
    border-radius: 4px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 8px;
    border: none;
    transition: background-color 0.2s;
  }

  .btn-secondary {
    background-color: #f3f4f6;
    color: #374151;
  }

  .btn-secondary:hover {
    background-color: #e5e7eb;
  }

  .btn-primary {
    background-color: #3b82f6;
    color: white;
  }

  .btn-primary:hover {
    background-color: #2563eb;
  }
</style>
