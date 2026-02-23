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
  } = $props<{
    currentPage?: number;
    maxPage?: number;
    rowCount?: number;
    gotoNext?: () => void;
    gotoPrev?: () => void;
    gotoFirst?: () => void;
    gotoLast?: () => void;
  }>();

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
      <button class="icon-btn" aria-label="Add Row">
        <i class="fa-solid fa-plus"></i>
      </button>
      <button class="icon-btn" aria-label="Delete Row">
        <i class="fa-solid fa-minus"></i>
      </button>
      <div class="divider"></div>
      <button class="icon-btn" aria-label="Upload">
        <i class="fa-solid fa-upload"></i>
      </button>
      <button class="icon-btn" aria-label="Export">
        <i class="fa-solid fa-download"></i>
        <!-- Replaced with download/export -->
      </button>
      <div class="divider"></div>
      <button class="icon-btn" aria-label="View Option">
        <i class="fa-solid fa-eye"></i>
      </button>
      <button class="icon-btn" aria-label="Refresh">
        <i class="fa-solid fa-rotate-right"></i>
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
        <span>100 rows</span>
        <i class="fa-solid fa-chevron-down"></i>
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
</style>
