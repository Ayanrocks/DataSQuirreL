<script lang="ts">
  import { onMount } from "svelte";

  let {
    initialValue = "",
    onCommit,
    onCancel,
  } = $props<{
    initialValue?: string;
    onCommit: (val: string) => void;
    onCancel: () => void;
  }>();

  let editValue = $state(initialValue);
  let inputRef = $state<HTMLInputElement | null>(null);

  onMount(() => {
    if (inputRef) {
      inputRef.focus();
      inputRef.select();
    }
  });

  function handleBlur() {
    onCommit(editValue);
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === "Enter") {
      onCommit(editValue);
    } else if (e.key === "Escape") {
      onCancel();
    }
  }

  function stopPropagation(e: Event) {
    e.stopPropagation();
  }
</script>

<input
  bind:this={inputRef}
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
</style>
