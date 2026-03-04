<script lang="ts">
  import { onMount } from "svelte";
  import { isShortcut, Shortcuts } from "../lib/shortcuts";

  let {
    initialValue = "",
    onCommit,
    onCancel,
    rawType = "date",
  } = $props<{
    initialValue?: string;
    onCommit: (val: string, direction: "none" | "next" | "down") => void;
    onCancel: () => void;
    rawType?: string;
  }>();

  // Try to parse initial value
  let parsedDate = initialValue ? new Date(initialValue) : new Date();
  if (isNaN(parsedDate.getTime())) {
    parsedDate = new Date(); // fallback to now if invalid
  }

  // Component State
  let year = $state(parsedDate.getFullYear());
  let month = $state(parsedDate.getMonth() + 1); // 1-12
  let day = $state(parsedDate.getDate());
  let hour = $state(parsedDate.getHours());
  let minute = $state(parsedDate.getMinutes());
  let second = $state(parsedDate.getSeconds());

  let containerRef = $state<HTMLDivElement | null>(null);

  onMount(() => {
    // Focus the container to catch keyboard events
    if (containerRef) {
      containerRef.focus();
    }
  });

  // Derived arrays for dropdowns
  let years = $derived(
    Array.from({ length: 100 }, (_, i) => new Date().getFullYear() - 50 + i),
  );
  let months = Array.from({ length: 12 }, (_, i) => i + 1);
  let daysInMonth = $derived(new Date(year, month, 0).getDate());
  let days = $derived(Array.from({ length: daysInMonth }, (_, i) => i + 1));
  let hours = Array.from({ length: 24 }, (_, i) => i);
  let minutes = Array.from({ length: 60 }, (_, i) => i);
  let seconds = Array.from({ length: 60 }, (_, i) => i);

  let hasTimeConfig = $derived(
    rawType.includes("time") || rawType.includes("timestamp"),
  );

  function pad(n: number) {
    return n.toString().padStart(2, "0");
  }

  function getFormattedDate() {
    let dateStr = `${year}-${pad(month)}-${pad(day)}`;
    if (hasTimeConfig) {
      dateStr += ` ${pad(hour)}:${pad(minute)}:${pad(second)}`;
      if (rawType.includes("tz")) {
        // basic timezone append if it's timestamptz
        // Not a perfect ISO string, but usually accepted by Postgres
        const offset = -new Date().getTimezoneOffset();
        const sign = offset >= 0 ? "+" : "-";
        const offHours = pad(Math.floor(Math.abs(offset) / 60));
        const offMinutes = pad(Math.abs(offset) % 60);
        dateStr += `${sign}${offHours}:${offMinutes}`;
      }
    }
    return dateStr;
  }

  function handleSave() {
    onCommit(getFormattedDate(), "none");
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (isShortcut(e, Shortcuts.Enter)) {
      e.preventDefault();
      onCommit(getFormattedDate(), "down");
    } else if (isShortcut(e, Shortcuts.Tab)) {
      e.preventDefault();
      onCommit(getFormattedDate(), "next");
    } else if (isShortcut(e, Shortcuts.Escape)) {
      e.preventDefault();
      onCancel();
    }
  }

  function stopPropagation(e: Event) {
    e.stopPropagation();
  }
</script>

<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div
  bind:this={containerRef}
  class="date-editor-container"
  tabindex="0"
  role="presentation"
  onkeydown={handleKeyDown}
  onclick={stopPropagation}
  onmousedown={stopPropagation}
>
  <div class="editor-header">
    <div class="title">Edit Date</div>
  </div>

  <div class="editor-body">
    <div class="date-group">
      <div class="flex flex-col gap-1">
        <label for="year-select">YYYY</label>
        <select id="year-select" bind:value={year}>
          {#each years as y}
            <option value={y}>{y}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-1">
        <label for="month-select">MM</label>
        <select id="month-select" bind:value={month}>
          {#each months as m}
            <option value={m}>{pad(m)}</option>
          {/each}
        </select>
      </div>

      <div class="flex flex-col gap-1">
        <label for="day-select">DD</label>
        <select id="day-select" bind:value={day}>
          {#each days as d}
            <option value={d}>{pad(d)}</option>
          {/each}
        </select>
      </div>
    </div>

    {#if hasTimeConfig}
      <div class="time-group mt-2">
        <div class="flex flex-col gap-1">
          <label for="hour-select">HH</label>
          <select id="hour-select" bind:value={hour}>
            {#each hours as h}
              <option value={h}>{pad(h)}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1">
          <label for="minute-select">mm</label>
          <select id="minute-select" bind:value={minute}>
            {#each minutes as m}
              <option value={m}>{pad(m)}</option>
            {/each}
          </select>
        </div>

        <div class="flex flex-col gap-1">
          <label for="second-select">ss</label>
          <select id="second-select" bind:value={second}>
            {#each seconds as s}
              <option value={s}>{pad(s)}</option>
            {/each}
          </select>
        </div>
      </div>
    {/if}
  </div>

  <div class="editor-footer">
    <button class="btn btn-cancel" onclick={onCancel}>Cancel</button>
    <button class="btn btn-save" onclick={handleSave}>Save</button>
  </div>
</div>

<style>
  .date-editor-container {
    position: absolute;
    top: 0;
    left: 0;
    min-width: 260px;
    background-color: white;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    box-shadow:
      0 10px 15px -3px rgba(0, 0, 0, 0.1),
      0 4px 6px -2px rgba(0, 0, 0, 0.05);
    z-index: 50;
    font-family: inherit;
    outline: none;
    display: flex;
    flex-direction: column;
  }

  .editor-header {
    background-color: #f3f4f6;
    padding: 8px 12px;
    border-bottom: 1px solid #e5e7eb;
    border-top-left-radius: 6px;
    border-top-right-radius: 6px;
  }

  .title {
    font-weight: 600;
    font-size: 13px;
    color: #374151;
  }

  .editor-body {
    padding: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .date-group,
  .time-group {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  label {
    font-size: 11px;
    color: #6b7280;
    font-weight: 500;
  }

  select {
    padding: 4px 6px;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    font-size: 13px;
    color: #111827;
    background-color: white;
    outline: none;
    cursor: pointer;
  }

  select:focus {
    border-color: #3b82f6;
    box-shadow: 0 0 0 1px #3b82f6;
  }

  .editor-footer {
    padding: 8px 12px;
    background-color: #f9fafb;
    border-top: 1px solid #e5e7eb;
    border-bottom-left-radius: 6px;
    border-bottom-right-radius: 6px;
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }

  .btn {
    padding: 4px 12px;
    font-size: 13px;
    font-weight: 500;
    border-radius: 4px;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.2s;
  }

  .btn-cancel {
    background-color: white;
    border-color: #d1d5db;
    color: #374151;
  }

  .btn-cancel:hover {
    background-color: #f3f4f6;
  }

  .btn-save {
    background-color: #3b82f6;
    color: white;
  }

  .btn-save:hover {
    background-color: #2563eb;
  }

  .flex {
    display: flex;
  }
  .flex-col {
    flex-direction: column;
  }
  .gap-1 {
    gap: 4px;
  }
  .mt-2 {
    margin-top: 8px;
  }
</style>
