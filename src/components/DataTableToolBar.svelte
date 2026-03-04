<script lang="ts">
  import { activeTable } from "../stores";

  let activeTableName = $state("");

  let {
    currentPage = 1,
    maxPage = 0,
    rowCount = 0,
    gotoNext = () => {},
    gotoPrev = () => {},
    gotoFirst = () => {},
    gotoLast = () => {},
    limit = 100,
    onLimitChange = () => {},
    isRefreshing = false,
    onRefresh = () => {},
    onAddRow = () => {},
    onRemoveRow = () => {},
    onCommit = () => {},
    onRevert = () => {},
    hasChanges = false,
  } = $props<{
    currentPage?: number;
    maxPage?: number;
    rowCount?: number;
    gotoNext?: () => void;
    gotoPrev?: () => void;
    gotoFirst?: () => void;
    gotoLast?: () => void;
    limit?: number | null;
    onLimitChange?: (val: number | null) => void;
    isRefreshing?: boolean;
    onRefresh?: () => void;
    onAddRow?: () => void;
    onRemoveRow?: () => void;
    onCommit?: () => void;
    onRevert?: () => void;
    hasChanges?: boolean;
  }>();

  let limitSelection = $state("100");
  let customLimitValue = $state(100);

  function handleSelectionChange(e: Event) {
    const val = (e.target as HTMLSelectElement).value;
    limitSelection = val;
    if (val === "All") {
      onLimitChange(null);
    } else if (val !== "Custom") {
      onLimitChange(parseInt(val, 10));
    } else {
      onLimitChange(customLimitValue);
    }
  }

  function handleCustomLimitChange(e: Event) {
    let val = parseInt((e.target as HTMLInputElement).value, 10);
    if (isNaN(val) || val < 1) val = 1;
    customLimitValue = val;
    onLimitChange(customLimitValue);
  }

  // Svelte 5 rune approach to subscription
  $effect(() => {
    const unsubscribe = activeTable.subscribe((value) => {
      activeTableName = value.tableName;
    });
    return unsubscribe;
  });

  let onClickNext = () => {
    if (currentPage >= maxPage && maxPage > 0) return;
    if (gotoNext) gotoNext();
  };

  let onClickPrev = () => {
    if (currentPage <= 1) return;
    if (gotoPrev) gotoPrev();
  };
</script>

<div class="data-table-toolbar__container">
  <div class="data-table-toolbar-controls__container">
    <div class="data-table-toolbar__controls--left">
      <button class="icon-btn" aria-label="Add Row" onclick={onAddRow}>
        <i class="fa-solid fa-plus"></i>
      </button>
      <button class="icon-btn" aria-label="Delete Row" onclick={onRemoveRow}>
        <i class="fa-solid fa-minus"></i>
      </button>
      <div class="divider"></div>
      <button
        class="icon-btn"
        aria-label="Commit Changes"
        title="Commit"
        onclick={onCommit}
        disabled={!hasChanges}
      >
        <i class="fa-solid fa-paper-plane"></i>
      </button>
      <button
        class="icon-btn"
        aria-label="Revert"
        title="Revert"
        onclick={onRevert}
        disabled={!hasChanges}
      >
        <i class="fa-solid fa-rotate-left"></i>
      </button>
      <button class="icon-btn" aria-label="Export">
        <i class="fa-solid fa-download"></i>
        <!-- Replaced with download/export -->
      </button>
      <div class="divider"></div>
      <button class="icon-btn" aria-label="View Option">
        <i class="fa-solid fa-eye"></i>
      </button>
      <button class="icon-btn" aria-label="Refresh" onclick={onRefresh}>
        <i class="fa-solid fa-rotate-right {isRefreshing ? 'fa-spin' : ''}"></i>
      </button>
      <button class="icon-btn" aria-label="Search">
        <i class="fa-solid fa-magnifying-glass"></i>
      </button>
      <div class="where-clause-container">
        <span class="where-label">WHERE</span>
        <input type="text" class="where-input" placeholder="" />
      </div>
    </div>

    <div class="data-table-toolbar__controls--right">
      <button
        class="icon-btn"
        onclick={gotoFirst}
        aria-label="First Page"
        disabled={currentPage <= 1}
      >
        <i class="fa-solid fa-backward-step"></i>
      </button>
      <button
        class="icon-btn"
        onclick={onClickPrev}
        aria-label="Previous Page"
        disabled={currentPage <= 1}
      >
        <i class="fa-solid fa-caret-left"></i>
      </button>

      <div class="row-count-selector">
        <select
          value={limitSelection}
          onchange={handleSelectionChange}
          class="pagination-select"
        >
          <option value="100">100 rows</option>
          <option value="1000">1000 rows</option>
          <option value="5000">5000 rows</option>
          <option value="10000">10000 rows</option>
          <option value="Custom">Custom</option>
          <option value="All">All</option>
        </select>
        {#if limitSelection === "Custom"}
          <input
            type="number"
            class="custom-limit-input"
            value={customLimitValue}
            onchange={handleCustomLimitChange}
            autocomplete="off"
          />
        {/if}
      </div>

      <button
        class="icon-btn"
        onclick={onClickNext}
        aria-label="Next Page"
        disabled={currentPage >= maxPage && maxPage > 0}
      >
        <i class="fa-solid fa-caret-right"></i>
      </button>
      <button
        class="icon-btn"
        onclick={gotoLast}
        aria-label="Last Page"
        disabled={currentPage >= maxPage && maxPage > 0}
      >
        <i class="fa-solid fa-forward-step"></i>
      </button>
    </div>
  </div>
</div>

<style>
  .data-table-toolbar__container {
    position: sticky;
    top: 0;
    left: 0;
    right: 0;
    height: 40px;
    background-color: #d1d5db; /* Light grey matching design */
    color: #374151; /* Darker grey for icons */
    font-size: 14px;
    border-bottom: 2px solid #9ca3af;
    z-index: 10;
    display: flex;
    align-items: center;
    padding: 0 10px;
  }

  .data-table-toolbar-controls__container {
    display: flex;
    justify-content: space-between;
    width: 100%;
    align-items: center;
  }

  .data-table-toolbar__controls--left,
  .data-table-toolbar__controls--right {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .icon-btn {
    background: transparent;
    border: none;
    color: #4b5563;
    cursor: pointer;
    padding: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.15s ease;
  }

  .icon-btn:hover:not(:disabled) {
    color: #111827;
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .divider {
    width: 1px;
    height: 16px;
    background-color: #9ca3af;
    margin: 0 4px;
  }

  .where-clause-container {
    display: flex;
    align-items: center;
    margin-left: 8px;
    background: transparent;
  }

  .where-label {
    font-weight: 600;
    color: #4b5563;
    margin-right: 8px;
    font-size: 13px;
  }

  .where-input {
    background: transparent;
    border: none;
    border-left: 2px solid #fca5a5; /* orange cursor indication */
    outline: none;
    width: 250px;
    color: #111827;
    font-family: inherit;
    font-size: 13px;
  }

  .where-input:focus {
    border-left: 2px solid #ef4444;
  }

  .row-count-selector {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: #4b5563;
    cursor: pointer;
    user-select: none;
  }

  .row-count-selector:hover {
    color: #111827;
  }

  .pagination-select {
    background: transparent;
    border: none;
    color: inherit;
    font-family: inherit;
    font-size: 12px;
    cursor: pointer;
    outline: none;
  }

  .pagination-select option {
    color: #111827;
  }

  .custom-limit-input {
    background: transparent;
    border: 1px solid #9ca3af;
    border-radius: 4px;
    width: 60px;
    padding: 2px 4px;
    font-size: 12px;
    color: #111827;
  }

  /* Remove arrows from number input */
  .custom-limit-input::-webkit-outer-spin-button,
  .custom-limit-input::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  .custom-limit-input[type="number"] {
    -moz-appearance: textfield;
    appearance: textfield;
  }
</style>
