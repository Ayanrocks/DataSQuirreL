<script lang="ts">
  import { fade } from "svelte/transition";

  let {
    show = false,
    queries = [],
    onClose = () => {},
    onSubmit = () => {},
  } = $props<{
    show: boolean;
    queries: string[];
    onClose: () => void;
    onSubmit: () => void;
  }>();

  let isCopied = $state(false);

  function copyToClipboard() {
    const textToCopy = queries.join("\n\n");
    navigator.clipboard
      .writeText(textToCopy)
      .then(() => {
        isCopied = true;
        setTimeout(() => {
          isCopied = false;
        }, 2000);
      })
      .catch((err) => {
        console.error("Failed to copy queries: ", err);
      });
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      onClose();
    }
  }

  function handleSubmit() {
    onSubmit();
  }
</script>

{#if show}
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    transition:fade={{ duration: 150 }}
  >
    <div class="modal-content">
      <div class="modal-header">
        <h2 class="modal-title">Query Preview</h2>
        <div class="modal-actions">
          <button
            class="icon-btn copy-btn"
            onclick={copyToClipboard}
            title="Copy to clipboard"
          >
            {#if isCopied}
              <i class="fa-solid fa-check text-green-500"></i>
            {:else}
              <i class="fa-regular fa-copy"></i>
            {/if}
          </button>
          <button class="icon-btn close-btn" onclick={onClose} title="Close">
            <i class="fa-solid fa-xmark"></i>
          </button>
        </div>
      </div>

      <div class="modal-body">
        {#if queries.length === 0}
          <div class="no-queries">No preview queries available.</div>
        {:else}
          <div class="code-container">
            <pre><code>{queries.join("\n\n")}</code></pre>
          </div>
        {/if}
      </div>

      <div class="modal-footer">
        <button class="btn btn-secondary" onclick={onClose}>Cancel</button>
        <button
          class="btn btn-primary"
          onclick={handleSubmit}
          disabled={queries.length === 0}
        >
          <i class="fa-solid fa-paper-plane mr-2"></i> Submit
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background-color: #ffffff;
    border-radius: 8px;
    width: 600px;
    max-width: 90vw;
    max-height: 80vh;
    display: flex;
    flex-direction: column;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    overflow: hidden;
  }

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid #e5e7eb;
    background-color: #f9fafb;
  }

  .modal-title {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
    color: #111827;
  }

  .modal-actions {
    display: flex;
    gap: 8px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: #4b5563;
    cursor: pointer;
    font-size: 16px;
    padding: 4px;
    border-radius: 4px;
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover {
    background-color: #e5e7eb;
    color: #111827;
  }

  .copy-btn {
    font-size: 14px;
  }

  .text-green-500 {
    color: #10b981 !important;
  }

  .modal-body {
    padding: 20px;
    overflow-y: auto;
    flex-grow: 1;
    background-color: #f3f4f6;
  }

  .no-queries {
    text-align: center;
    color: #6b7280;
    font-style: italic;
    padding: 20px 0;
  }

  .code-container {
    background-color: #1f2937;
    border-radius: 6px;
    padding: 16px;
    overflow-x: auto;
  }

  pre {
    margin: 0;
    font-family: "Jetbrains Mono", "Courier New", Courier, monospace;
    font-size: 13px;
    line-height: 1.5;
    color: #e5e7eb;
  }

  code {
    font-family: inherit;
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    padding: 16px 20px;
    border-top: 1px solid #e5e7eb;
    background-color: #f9fafb;
  }

  .btn {
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    border: none;
    transition: all 0.2s;
    display: flex;
    align-items: center;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    background-color: #e5e7eb;
    color: #374151;
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: #d1d5db;
  }

  .btn-primary {
    background-color: #2563eb;
    color: white;
  }

  .btn-primary:hover:not(:disabled) {
    background-color: #1d4ed8;
  }

  .mr-2 {
    margin-right: 8px;
  }
</style>
