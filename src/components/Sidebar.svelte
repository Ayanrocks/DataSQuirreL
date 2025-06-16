<script lang="ts">
  import { activeTable, notificationMsg, tableNames } from "../stores";
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import {
    NOTIFICATION_TYPE_ERROR,
    PAGINATION_SIZE,
  } from "../constants/constants";
  import SidebarToolbar from "./SidebarToolbar.svelte";
  import SideBarItem from "./SideBarItem.svelte";
  import RecursiveSidebarItem from "./RecursiveSidebarItem.svelte";

  const dispatch = createEventDispatcher();
  let activeTableName: string = "";

  function resize(e: MouseEvent) {
    dispatch("resizing", {
      event: e,
    });
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
    // checking if the same table is opened or not
    if (activeTableName == tableName) {
      // no change needed
      return;
    }
    // Invoke the command
    invoke("fetch_table_data", {
      reqPayload: {
        table_name: tableName,
      },
    })
      .then((res: any) => {
        // TODO: Define a proper type for res
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

        // calculate the total page numbers
        activeTableData.maxPage = Math.floor(
          res.data.row_count / PAGINATION_SIZE,
        );
        if (res.data.row_count % PAGINATION_SIZE) {
          activeTableData.maxPage++;
        }

        activeTable.set(activeTableData);
      })
      .catch((e: any) => {
        // TODO: Define a proper type for e
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

  function renderSideBarItem(item: any, mockTableData: any[], level: number): SidebarItem[] {
    const items: SidebarItem[] = [];
    
    for (const currentItem of mockTableData) {
      console.log(currentItem);
      items.push({
        entityName: currentItem.entityName,
        isExpanded: currentItem.isExpanded,
        entityType: currentItem.entityType,
        level: level,
        children: currentItem.children ? renderSideBarItem(currentItem, currentItem.children, level + 1) : []
      });
    }

    return items;
  }
</script>

<div class="column is-one-quarter split-sidebar" id="left-sidebar">
  <button
    class="split-sidebar-draggable-div"
    aria-label="Resize sidebar"
    onmousedown={resize}
  >
    <span id="resize-icon">
      <i class="fas fa-solid fa-grip-lines-vertical"></i>
    </span>
  </button>
  <div class="sidebar-content">
    <SidebarToolbar />
    <div class="table-list has-text-left">
      {#each renderSideBarItem(null, MockTableData, 0) as item}
        <RecursiveSidebarItem {item} />
      {/each}
    </div>
  </div>
</div>

<style>
  .split-sidebar {
    height: 100%;
    width: 100%;
    flex: none;
    color: var(--offWhite);
    background-color: var(--tertiaryColor);
  }

  .sidebar-content {
    display: flex;
    flex-direction: column;
    font-size: 12px;
    font-weight: 600;
    height: 100%;
  }

  .split-sidebar-draggable-div {
    position: absolute;
    right: 0;
    background-color: var(--secondaryColor);
    width: 10px;
    height: 100%;
    margin-left: 30px;
    cursor: ew-resize;
  }

  .table-list {
    overflow-y: scroll;
    overflow-x: hidden;
    margin: 20px 5px;
    line-height: 25px;
    word-break: break-all;
  }

  #resize-icon {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 14px;
    color: var(--yellowPrimary);
  }
</style>
