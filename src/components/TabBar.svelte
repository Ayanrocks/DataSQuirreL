<script lang="ts">
  import type { ActiveTable } from "../stores";

  let {
    tabs = [],
    activeTabIndex = -1,
    onTabChange = (index: number) => {},
    onTabClose = (index: number) => {},
  } = $props<{
    tabs: ({ id: string } & ActiveTable)[];
    activeTabIndex: number;
    onTabChange: (index: number) => void;
    onTabClose: (index: number) => void;
  }>();
</script>

<div class="tabbar-container">
  {#each tabs as tab, i (tab.id)}
    <div class="tab-item {i === activeTabIndex ? 'active' : ''}">
      <button
        class="tab-label"
        onclick={() => onTabChange(i)}
        onkeydown={(e) => {
          if (e.key === "Enter") onTabChange(i);
        }}
      >
        <i class="fa-solid fa-table tab-icon"></i>
        <span class="tab-text">{tab.tableName}</span>
      </button>
      <button
        class="tab-close"
        onclick={(e) => {
          e.stopPropagation();
          onTabClose(i);
        }}
        aria-label="Close tab"
      >
        <i class="fa-solid fa-xmark"></i>
      </button>
    </div>
  {/each}
</div>

<style>
  .tabbar-container {
    display: flex;
    flex-direction: row;
    align-items: flex-end; /* Tabs sit at the bottom of the container */
    background-color: #f3f4f6; /* Lighter background matching inactive tabs */
    height: 36px;
    min-height: 36px;
    width: 100%;
    overflow-x: auto; /* Allow horizontal scrolling for many tabs */
    border-bottom: 1px solid #d1d5db;
    box-sizing: border-box;
  }

  .tab-item {
    display: flex;
    align-items: center;
    background-color: #e5e7eb;
    color: #4b5563;
    border: 1px solid #d1d5db;
    border-bottom: none;
    border-radius: 6px 6px 0 0;
    padding: 4px 10px;
    margin-right: -1px; /* Overlap borders slightly */
    height: 30px;
    min-width: 120px;
    max-width: 200px;
    cursor: pointer;
    position: relative;
    box-sizing: border-box;
    transition: background-color 0.15s ease;
  }

  .tab-item:hover {
    background-color: #f9fafb;
  }

  .tab-item.active {
    background-color: #ffffff;
    color: #111827;
    font-weight: 500;
    z-index: 2; /* Bring active tab to top */
    height: 32px; /* Slightly taller */
  }

  /* Create the illusion that the active tab merges with the content below */
  .tab-item.active::after {
    content: "";
    position: absolute;
    bottom: -1px;
    left: 0;
    right: 0;
    height: 2px;
    background-color: #ffffff;
    z-index: 3;
  }

  .tab-label {
    background: transparent;
    border: none;
    color: inherit;
    font-family: inherit;
    font-size: 13px;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    overflow: hidden;
    text-align: left;
    padding: 0;
  }

  .tab-icon {
    color: #6b7280;
    font-size: 12px;
  }

  .tab-item.active .tab-icon {
    color: #2563eb; /* Blue highlight for active icon */
  }

  .tab-text {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tab-close {
    background: transparent;
    border: none;
    color: #9ca3af;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    border-radius: 4px;
    padding: 0;
    margin-left: 6px;
  }

  .tab-close:hover {
    background-color: #d1d5db;
    color: #1f2937;
  }

  .tabbar-container::-webkit-scrollbar {
    height: 4px;
  }

  .tabbar-container::-webkit-scrollbar-thumb {
    background-color: #9ca3af;
    border-radius: 4px;
  }
</style>
