<script lang="ts">
  import { activeTable, notificationMsg, tableNames } from "../stores";
  import { invoke } from "@tauri-apps/api/core";
  import {
    NOTIFICATION_TYPE_ERROR,
    PAGINATION_SIZE,
  } from "../constants/constants";
  import SidebarToolbar from "./SidebarToolbar.svelte";
  import RecursiveSidebarItem from "./RecursiveSidebarItem.svelte";
  import type { SidebarItem, SchemaData } from "../types/interface";

  let activeTableName: string = "";

  export let dashboardData: SchemaData[];

  // Debug logging
  $: console.log('Sidebar dashboardData:', dashboardData);
  $: console.log('Sidebar dashboardData length:', dashboardData?.length);
  $: console.log('Sidebar dashboardData type:', typeof dashboardData);
  $: console.log('Sidebar dashboardData is array:', Array.isArray(dashboardData));



  // Sidebar width state
  let sidebarWidth = 260; // px, default width
  const minSidebarWidth = 180;
  const maxSidebarWidth = 500;
  let isResizing = false;
  let sidebarRef: HTMLDivElement;

  function handleMouseDown(e: MouseEvent) {
    isResizing = true;
    document.body.style.cursor = "ew-resize";
    document.body.style.userSelect = "none";
    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isResizing) return;
    const newWidth = Math.min(
      Math.max(e.clientX, minSidebarWidth),
      maxSidebarWidth,
    );
    // Direct DOM manipulation for instant feedback
    if (sidebarRef) {
      sidebarRef.style.width = `${newWidth}px`;
    }
  }

  function handleMouseUp(e: MouseEvent) {
    isResizing = false;
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("mouseup", handleMouseUp);
    // Sync the width back to Svelte state
    if (sidebarRef) {
      const width = parseInt(sidebarRef.style.width, 10);
      sidebarWidth = width;
    }
  }

  let sideBarColumn: string = "Table Names";
  let tables: string[] = [];

  tableNames.subscribe((e) => {
    tables = e.tables;
    sideBarColumn = e.tableName;
  });

  activeTable.subscribe((val) => {
    activeTableName = val.tableName;
  });

  // let MockTableData = [
  //   {
  //     entityType: "postgresql",
  //     entityName: "Database",
  //     isExpanded: true,
  //     children: [
  //       {
  //         entityType: "Schema",
  //         entityName: "Public",
  //         isExpanded: true,
  //         children: [
  //           {
  //             entityType: "Table",
  //             entityName: "users",
  //             isExpanded: false,
  //           },
  //           {
  //             entityType: "Table",
  //             entityName: "posts",
  //             isExpanded: false,
  //           },
  //         ],
  //       },
  //     ],
  //   },
  // ];

  function renderSideBarItem(
    data: SchemaData[],
    level: number,
  ): SidebarItem[] {
    console.log('renderSideBarItem called with data:', data, 'level:', level);
    const items: SidebarItem[] = [];
    for (const currentItem of data) {
      console.log('Processing item:', currentItem);
      items.push({
        entityName: currentItem.entityName,
        isExpanded: currentItem.isExpanded,
        entityType: currentItem.entityType,
        level: level,
        children: currentItem.children && currentItem.children.length > 0
          ? renderSideBarItem(currentItem.children, level + 1)
          : [],
      });
    }
    console.log('renderSideBarItem returning items:', items);
    return items;
  }
</script>

<div
  bind:this={sidebarRef}
  class="sidebar-resize-wrapper"
  style="width: {sidebarWidth}px; min-width: {minSidebarWidth}px; max-width: {maxSidebarWidth}px;"
>
  <div class="sidebar-content">
    <SidebarToolbar />
    <div class="table-list has-text-left">
      {#if dashboardData && dashboardData.length > 0}
        {#each renderSideBarItem(dashboardData, 0) as item}
          <RecursiveSidebarItem {item} />
        {/each}
      {:else}
        <div class="p-4 text-gray-500">
          {dashboardData ? 'No data available' : 'Loading...'}
        </div>
      {/if}
    </div>
  </div>
  <button
    class="sidebar-divider"
    on:mousedown={handleMouseDown}
    aria-label="Resize sidebar"
  >
    <span class="resize-icon">
      <i class="fas fa-solid fa-grip-lines-vertical"></i>
    </span>
  </button>
</div>

<style>
  .sidebar-resize-wrapper {
    display: flex;
    flex-direction: row;
    height: 100%;
    background-color: var(--tertiaryColor);
    position: relative;
    z-index: 2;
    margin-top: 50px;
  }
  .sidebar-content {
    display: flex;
    flex-direction: column;
    font-size: 12px;
    font-weight: 600;
    height: 100%;
    flex: 1;
    min-width: 0;
  }
  .sidebar-divider {
    width: 4px;
    min-width: 4px;
    max-width: 8px;
    cursor: ew-resize;
    background-color: var(--accentColor);
    display: flex;
    align-items: center;
    justify-content: center;
    user-select: none;
    height: 100%;
    z-index: 10;
    transition: background 0.15s;
    touch-action: none;
  }

  .resize-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    color: var(--yellowPrimary);
    width: 100%;
    height: 24px;
  }
  .table-list {
    overflow-y: scroll;
    overflow-x: hidden;
    margin: 20px 5px;
    line-height: 25px;
    word-break: break-all;
    height: 85%;
  }

</style>
