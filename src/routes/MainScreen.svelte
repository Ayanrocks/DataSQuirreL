<script lang="ts">
  export const ssr = false;

  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onDestroy, onMount } from "svelte";
  import Sidebar from "../components/Sidebar.svelte";
  import DataTable from "../components/DataTable.svelte";
  import {
    notificationMsg,
    tableNames,
    windowWidth,
    windowHeight,
    type ActiveTable,
  } from "../stores";
  import {
    NOTIFICATION_TYPE_ERROR,
    BORDER_SIZE,
    MAX_RESIZE_EXPANDABLE_SIZE,
    MIN_RESIZE_EXPANDABLE_SIZE,
    INVOKE_GET_TABLE_DATA,
    INVOKE_FETCH_TABLE_DATA_WITH_OFFSET,
  } from "../constants/constants";
  import MainTopBar from "../components/MainTopBar.svelte";
  import type { IPCResponse, DashboardData } from "../types/response";
  import type { SchemaData, SidebarItem } from "../types/interface";
  import TabBar from "../components/TabBar.svelte";

  const appWindow = getCurrentWindow();

  let activeConnectionName = $state("");
  let dashboardData: SchemaData[] = $state([]);

  var unlisten: any;

  let tabs = $state<({ id: string } & ActiveTable)[]>([]);
  let activeTabIndex = $state(-1);

  onDestroy(async () => {
    if (unlisten) {
      if (typeof unlisten === "function") {
        unlisten();
      } else if (unlisten.then) {
        const fn = await unlisten;
        fn();
      }
    }
  });

  // Replaced activeTable.subscribe with handleTableSelect
  function handleTableSelect(entityType: string, fullPath: string) {
    if (entityType === "Table") {
      let dbComponents = fullPath.split("::");
      let dbName = dbComponents[0];
      let schemaName = dbComponents[1];
      let tableName = dbComponents[2];

      let tabId = `${dbName}::${schemaName}::${tableName}`;

      // Check if tab already exists
      let existingIndex = tabs.findIndex((t) => t.id === tabId);
      if (existingIndex !== -1) {
        activeTabIndex = existingIndex;
        return;
      }

      // Create new tab
      tabs.push({
        id: tabId,
        tableName: tableName,
        schemaName: schemaName,
        dbName: dbName,
        displayName: tableName,
        columns: [],
        currentPage: 1,
        maxPage: 0,
        rowCount: 0,
        rows: [],
      });
      activeTabIndex = tabs.length - 1;
      updateTabNames();

      // Fetch initial data
      invokeTableData(dbName, schemaName, tableName, activeTabIndex, 0, 100);
    }
  }

  function handleTabChange(index: number) {
    activeTabIndex = index;
  }

  function handleTabClose(index: number) {
    tabs.splice(index, 1);
    if (tabs.length === 0) {
      activeTabIndex = -1;
    } else if (activeTabIndex === index) {
      activeTabIndex = Math.max(0, index - 1);
    } else if (activeTabIndex > index) {
      activeTabIndex--;
    }
    updateTabNames();
  }

  function updateTabNames() {
    for (let tab of tabs) {
      let conflicts = tabs.filter(
        (t) => t.tableName === tab.tableName && t.id !== tab.id,
      );
      if (conflicts.length === 0) {
        tab.displayName = tab.tableName;
      } else {
        let schemaConflicts = conflicts.filter(
          (t) => t.schemaName === tab.schemaName,
        );
        if (schemaConflicts.length === 0) {
          tab.displayName = `${tab.schemaName}.${tab.tableName}`;
        } else {
          tab.displayName = `${tab.dbName}.${tab.schemaName}.${tab.tableName}`;
        }
      }
    }
  }

  onMount(async () => {
    // Get the current window label and extract the connection name
    const label = await appWindow.label;
    activeConnectionName = label.replace(/^connection-window-/, "");
    console.log("Window label:", label);
    console.log("Active connection name:", activeConnectionName);

    unlisten = appWindow.onResized(async () => {
      const factor = await appWindow.scaleFactor();
      const position = await appWindow.innerSize();
      let logicalSize = position.toLogical(factor);

      windowWidth.set(logicalSize.width);
      windowHeight.set(logicalSize.height);
    });

    // setting the current window height
    appWindow.innerSize().then(async (w) => {
      const factor = await appWindow.scaleFactor();
      let logicalSize = w.toLogical(factor);

      windowWidth.set(logicalSize.width);
      windowHeight.set(logicalSize.height);
    });

    // fetch tables on load
    try {
      console.log("Fetching dashboard data for window label:", appWindow.label);

      const res = await invoke<IPCResponse<DashboardData>>(
        "fetch_dashboard_data",
        {
          reqPayload: {
            connection_window_label: appWindow.label,
          },
        },
      );

      console.log("API response:", res);

      if (res.error_code) {
        console.error("API error:", res.error_code, res.frontend_msg);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: res.frontend_msg || "An error occurred",
        });
        return;
      }

      if (res.data) {
        dashboardData = res.data.dashboard_data;
        console.log("MainScreen dashboardData:", dashboardData);
      }
    } catch (e) {
      console.log(e);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "Something went wrong. Check console for more information",
      });
    }
  });

  async function invokeTableData(
    databaseName: string,
    schema: string,
    tableName: string,
    tabIndex: number,
    offset: number = 0,
    limit: number | null = 100,
    sortColumn: string | null = null,
    sortDirection: string | null = null,
    whereClause: string | null = null,
  ) {
    try {
      const res = await invoke<IPCResponse<any>>(
        INVOKE_FETCH_TABLE_DATA_WITH_OFFSET,
        {
          reqPayload: {
            database_name: databaseName,
            schema_name: schema,
            table_name: tableName,
            offset: offset,
            limit: limit,
            sort_column: sortColumn,
            sort_direction: sortDirection,
            where_clause: whereClause,
          },
        },
      );

      console.log("RES:::", res);
      if (res.error_code) {
        throw new Error(res.frontend_msg || res.sys_err || "Query failed");
      }
      if (tabs[tabIndex]) {
        tabs[tabIndex].rows = res.data.rows;
        tabs[tabIndex].columns = res.data.columns;
        tabs[tabIndex].rowCount = parseInt(res.data.row_count);
        // We manage virtual pages differently now if needed, but let's keep compatibility
        tabs[tabIndex].currentPage =
          res.data.current_page || Math.floor(offset / (limit || 100)) + 1;
        tabs[tabIndex].maxPage =
          res.data.max_page ||
          (limit ? Math.ceil(tabs[tabIndex].rowCount / limit) : 1);

        // Also ensure limit/offset can be read if stored on tab
        (tabs[tabIndex] as any).currentOffset = offset;
        (tabs[tabIndex] as any).currentLimit = limit;
        if (res.data.primary_keys) {
          tabs[tabIndex].primaryKeys = res.data.primary_keys;
        }
        if (res.data.foreign_keys) {
          tabs[tabIndex].foreignKeys = res.data.foreign_keys;
        }
      }
    } catch (e: any) {
      console.log("invokeTableData ErrorCatching: ", e);
      throw e;
    }
  }
</script>

<div class="main-container">
  <MainTopBar connectionName={activeConnectionName} />
  <Sidebar {dashboardData} onTableSelect={handleTableSelect} />

  <div class="right-panes">
    <TabBar
      {tabs}
      {activeTabIndex}
      onTabChange={handleTabChange}
      onTabClose={handleTabClose}
    />
    <div class="columns split-main-content" id="right-main-content">
      {#each tabs as tab, i (tab.id)}
        <div
          class="tab-content"
          style="display: {i === activeTabIndex
            ? 'block'
            : 'none'}; height: 100%; width: 100%;"
        >
          <DataTable
            bind:activeTableData={tabs[i]}
            fetchData={(
              offset,
              limit,
              sortColumn,
              sortDirection,
              whereClause,
            ) =>
              invokeTableData(
                tab.dbName,
                tab.schemaName,
                tab.tableName,
                i,
                offset,
                limit,
                sortColumn,
                sortDirection,
                whereClause,
              )}
          />
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .main-container {
    height: 100vh;
    width: 100vw;
    background-color: var(--offWhite);
    color: var(--accentColor);
    display: flex;
    justify-content: flex-start;
    overflow: hidden;
  }

  .right-panes {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
    margin-top: 50px;
    height: calc(100vh - 50px);
    background-color: #ffffff;
  }

  .split-main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
</style>
