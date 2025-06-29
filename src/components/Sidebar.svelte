<script lang="ts">
  import { activeTable, notificationMsg, tableNames } from "../stores";
  import { invoke } from "@tauri-apps/api/core";
  import {
    NOTIFICATION_TYPE_ERROR,
    PAGINATION_SIZE,
  } from "../constants/constants";
  import SidebarToolbar from "./SidebarToolbar.svelte";
  import RecursiveSidebarItem from "./RecursiveSidebarItem.svelte";

  let activeTableName: string = "";

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
  let MockTableData = [
    {
      entityType: "postgresql",
      entityName: "Database",
      isExpanded: true,
      children: [
        {
          entityType: "Schema",
          entityName: "Public",
          isExpanded: true,
          children: [
            {
              entityType: "Table",
              entityName: "users",
              isExpanded: false,
            },
            {
              entityType: "Table",
              entityName: "posts",
              isExpanded: false,
            },
          ],
        },
      ],
    },
  ];

  tableNames.subscribe((e) => {
    tables = e.tables;
    sideBarColumn = e.tableName;
  });

  activeTable.subscribe((val) => {
    activeTableName = val.tableName;
  });

  function clickedSidebar(tableName: string) {
    if (activeTableName == tableName) {
      return;
    }
    invoke("fetch_table_data", {
      reqPayload: {
        table_name: tableName,
      },
    })
      .then((res: any) => {
        console.log(res);
        activeTableName = res.data.table_name;
        let activeTableData = {
          tableName: res.data.table_name,
          columns: res.data.columns,
          rows: res.data.rows,
          rowCount: res.data.row_count,
          currentPage: 1,
          maxPage: 0,
        };
        activeTableData.maxPage = Math.floor(
          res.data.row_count / PAGINATION_SIZE,
        );
        if (res.data.row_count % PAGINATION_SIZE) {
          activeTableData.maxPage++;
        }
        activeTable.set(activeTableData);
      })
      .catch((e: any) => {
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: e,
        });
      });
  }

  interface SidebarItem {
    entityName: string;
    isExpanded: boolean;
    entityType: string;
    level: number;
    children: SidebarItem[];
  }

  function renderSideBarItem(
    item: any,
    mockTableData: any[],
    level: number,
  ): SidebarItem[] {
    const items: SidebarItem[] = [];
    for (const currentItem of mockTableData) {
      items.push({
        entityName: currentItem.entityName,
        isExpanded: currentItem.isExpanded,
        entityType: currentItem.entityType,
        level: level,
        children: currentItem.children
          ? renderSideBarItem(currentItem, currentItem.children, level + 1)
          : [],
      });
    }
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
      {#each renderSideBarItem(null, MockTableData, 0) as item}
        <RecursiveSidebarItem {item} />
      {/each}
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
  #resize-icon {
    display: none;
  }
</style>
