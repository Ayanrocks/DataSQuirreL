<script lang="ts">
  import { onMount } from "svelte";
  import DateEditor from "./DateEditor.svelte";
  import { isShortcut, Shortcuts } from "../lib/shortcuts";

  let {
    initialValue = "",
    onCommit,
    onCancel,
    jsType = "string",
    rawType = "text",
  } = $props<{
    initialValue?: string;
    onCommit: (val: string, direction: "none" | "next" | "down") => void;
    onCancel: () => void;
    jsType?: string;
    rawType?: string;
  }>();

  let editValue = $state(initialValue);
  let inputRef = $state<HTMLInputElement | HTMLTextAreaElement | null>(null);

  onMount(() => {
    if (inputRef) {
      inputRef.focus({ preventScroll: true });
      inputRef.select();
    }
  });

  function handleBlur() {
    onCommit(editValue, "none");
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (isShortcut(e, Shortcuts.Enter)) {
      e.preventDefault();
      onCommit(editValue, "down");
    } else if (isShortcut(e, Shortcuts.Tab)) {
      e.preventDefault();
      onCommit(editValue, "next");
    } else if (isShortcut(e, Shortcuts.Escape)) {
      onCancel();
    }
  }

  function stopPropagation(e: Event) {
    e.stopPropagation();
  }

  let isJson = $derived(
    jsType === "object" || rawType === "json" || rawType === "jsonb",
  );
  let isDate = $derived(
    jsType === "Date" || rawType === "timestamp" || rawType === "date",
  );
</script>

{#if isDate}
  <DateEditor {initialValue} {onCommit} {onCancel} {rawType} />
{:else if isJson}
  <textarea
    bind:this={inputRef as HTMLTextAreaElement}
    bind:value={editValue}
    onblur={handleBlur}
    onkeydown={handleKeyDown}
    onclick={stopPropagation}
    onmousedown={stopPropagation}
    class="cell-editor-input cell-editor-textarea"
    autocomplete="off"
    autocapitalize="off"
    spellcheck="false"
  ></textarea>
{:else}
  <input
    bind:this={inputRef as HTMLInputElement}
    bind:value={editValue}
    onblur={handleBlur}
    onkeydown={handleKeyDown}
    onclick={stopPropagation}
    onmousedown={stopPropagation}
    class="cell-editor-input"
    autocomplete="off"
    autocorrect="off"
    autocapitalize="off"
    spellcheck="false"
  />
{/if}

<style>
  .cell-editor-input {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    height: 100%;
    margin: 0;
    padding: inherit;
    border: none;
    outline: 2px solid #3b82f6;
    outline-offset: -2px;
    background-color: white;
    font-family: inherit;
    font-size: inherit;
    color: inherit;
    box-sizing: border-box;
    z-index: 10;
  }

  .cell-editor-textarea {
    resize: none;
    line-height: inherit;
    word-break: break-all;
    white-space: pre-wrap;
    overflow-y: auto;
  }
</style>
