<script lang="ts">
  let {
    isExecuting = false,
    executionTimeMs = null,
    hasUnsavedChanges = false,
    hasSelection = false,
    onRun = () => {},
    onRunSelection = () => {},
    onFormat = () => {},
    onSave = () => {},
  } = $props<{
    isExecuting?: boolean;
    executionTimeMs?: number | null;
    hasUnsavedChanges?: boolean;
    hasSelection?: boolean;
    onRun?: () => void;
    onRunSelection?: () => void;
    onFormat?: () => void;
    onSave?: () => void;
  }>();
</script>

<div class="console-toolbar__container">
  <div class="console-toolbar-controls__container">
    <div class="console-toolbar__controls--left">
      <button
        class="icon-btn play-btn"
        aria-label="Run Query"
        title="Run Query (Cmd/Ctrl + Enter)"
        onclick={onRun}
        disabled={isExecuting}
      >
        {#if isExecuting}
          <i class="fa-solid fa-circle-notch fa-spin"></i>
        {:else}
          <i class="fa-solid fa-play"></i>
        {/if}
        <span class="btn-text">Run</span>
      </button>

      <button
        class="icon-btn"
        aria-label="Run Selection"
        title="Run Selection (Cmd/Ctrl + Shift + Enter)"
        onclick={onRunSelection}
        disabled={isExecuting || !hasSelection}
      >
        <i class="fa-solid fa-play selection-icon"></i>
        <span class="btn-text">Run Selection</span>
      </button>

      <div class="divider"></div>

      <button
        class="icon-btn"
        aria-label="Format Query"
        title="Format Query"
        onclick={onFormat}
      >
        <i class="fa-solid fa-align-left"></i>
      </button>

      <div class="divider"></div>

      <button
        class="icon-btn"
        aria-label="Save Console"
        title="Save (Cmd/Ctrl + S)"
        onclick={onSave}
        disabled={!hasUnsavedChanges}
      >
        <i class="fa-solid fa-save"></i>
      </button>
    </div>

    <div class="console-toolbar__controls--right">
      {#if executionTimeMs !== null}
        <div class="execution-time">
          <i class="fa-solid fa-clock"></i>
          <span>{executionTimeMs} ms</span>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .console-toolbar__container {
    height: 40px;
    background-color: #d1d5db;
    color: #374151;
    font-size: 14px;
    border-bottom: 2px solid #9ca3af;
    z-index: 10;
    display: flex;
    align-items: center;
    padding: 0 10px;
    flex-shrink: 0;
  }

  .console-toolbar-controls__container {
    display: flex;
    justify-content: space-between;
    width: 100%;
    align-items: center;
  }

  .console-toolbar__controls--left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .console-toolbar__controls--right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: #4b5563;
    cursor: pointer;
    padding: 6px 8px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    transition: all 0.15s ease;
  }

  .icon-btn:hover:not(:disabled) {
    background-color: rgba(255, 255, 255, 0.4);
    color: #111827;
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .play-btn {
    color: #059669; /* Green hue for the primary action */
    font-weight: 500;
  }

  .play-btn:hover:not(:disabled) {
    color: #047857;
  }

  .selection-icon {
    font-size: 0.8em;
    opacity: 0.8;
  }

  .btn-text {
    font-size: 13px;
  }

  .divider {
    width: 1px;
    height: 16px;
    background-color: #9ca3af;
    margin: 0 4px;
  }

  .execution-time {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #4b5563;
    font-family: "JetBrains Mono", monospace;
  }
</style>
