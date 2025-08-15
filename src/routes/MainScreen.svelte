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
    activeTable,
    type ActiveTable, // Import ActiveTable interface
  } from "../stores";
  import {
    NOTIFICATION_TYPE_ERROR,
    BORDER_SIZE,
    MAX_RESIZE_EXPANDABLE_SIZE,
    MIN_RESIZE_EXPANDABLE_SIZE,
  } from "../constants/constants";
  import MainTopBar from "../components/MainTopBar.svelte";
  import type { IPCResponse, DashboardData } from "../types/response";
  import type { SchemaData, SidebarItem } from "../types/interface";
  import TabBar from "../components/TabBar.svelte";
  
  const appWindow = getCurrentWindow();

  // on mousedown for the draggable

  let m_pos: number | undefined;
  let activeConnectionName = "";
  let dashboardData: SchemaData[] = [];

  /// to resize the window on drag
  function resize(e: MouseEvent) {
    if (m_pos === undefined) return; // Add null check
    const dx = e.x - m_pos;
    m_pos = e.x;
    const leftSidebarContainer = document.getElementById(
      "left-sidebar-container",
    );
    const rightMainContainer = document.getElementById("right-main-content");

    if (leftSidebarContainer && rightMainContainer) {
      // Add null checks
      let computedWidth =
        parseInt(getComputedStyle(leftSidebarContainer, "").width) + dx;
      let computedWidthInPx = computedWidth + "px";

      if (
        computedWidth <= MAX_RESIZE_EXPANDABLE_SIZE &&
        computedWidth >= MIN_RESIZE_EXPANDABLE_SIZE
      ) {
        leftSidebarContainer.style.width = computedWidthInPx;
        rightMainContainer.style.width = $windowWidth - computedWidth + "px";
        rightMainContainer.style.marginLeft = computedWidthInPx;
      }
    }
  }

  /// to resize the sidebar
  function resizeSideBar(e: CustomEvent) {
    const event = e.detail.event;
    if (event.offsetX < BORDER_SIZE) {
      m_pos = event.x;
      document.addEventListener("mousemove", resize, false);
    }
  }

  /// adding mousemove and mouseup event listeners
  document.addEventListener(
    "mouseup",
    function () {
      document.removeEventListener("mousemove", resize, false);
    },
    false,
  );

  var unlisten: any; // TODO: Find a more specific type for unlisten
  let activeTableData: ActiveTable = {
    // Initialize with the correct type
    tableName: "",
    rows: [[]],
    columns: [""],
    rowCount: 0,
    currentPage: 0,
    maxPage: 0,
  };

  onDestroy(() => {
    if (unlisten) {
      // Add null check
      unlisten();
    }
  });

  activeTable.subscribe((val) => {
    activeTableData = val;
  });

  onMount(async () => {
    // Get the current window label and extract the connection name
    const label = await appWindow.label;
    activeConnectionName = label.replace(/^connection-window-/, "");
    console.log('Window label:', label);
    console.log('Active connection name:', activeConnectionName);

    // on change of width, check and set the width of the main and sidebar content
    windowWidth.subscribe((val) => {
      // set initial width of sidebar and main content area
      const leftSidebarContainer = document.getElementById(
        "left-sidebar-container",
      );
      const rightMainContainer = document.getElementById("right-main-content");

      if (leftSidebarContainer && rightMainContainer) {
        // Add null checks
        let computedWidth = parseInt(
          getComputedStyle(leftSidebarContainer, "").width,
        );
        let computedWidthInPx = computedWidth + "px";

        leftSidebarContainer.style.width = computedWidthInPx;
        rightMainContainer.style.width = val - computedWidth + "px";
        rightMainContainer.style.marginLeft = computedWidthInPx;
      }
    });

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
      console.log('Fetching dashboard data for window label:', appWindow.label);

      const res = await invoke<IPCResponse<DashboardData>>(
        "fetch_dashboard_data",
        {
          reqPayload: {
            connection_window_label: appWindow.label,
          },
        },
      );

      console.log('API response:', res);
      
      if (res.error_code) {
        console.error('API error:', res.error_code, res.frontend_msg);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: res.frontend_msg || "An error occurred",
        });
        return;
      }

      if (res.data) {
        dashboardData = res.data.dashboard_data;
        console.log('MainScreen dashboardData:', dashboardData);
      }
    } catch (e) {
      console.log(e);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "Something went wrong. Check console for more information",
      });
    }
  });
</script>

<div class="main-container">
  <MainTopBar connectionName={activeConnectionName} />
  <Sidebar on:resizing={resizeSideBar} {dashboardData} />
  <TabBar />
  <div class="columns split-main-content" id="right-main-content">
    {#if activeTableData.tableName !== ""}
      <DataTable />
    {/if}
  </div>
</div>

<style>
  .main-container {
    height: 102vh;
    width: 100vw;
    background-color: var(--offWhite);
    color: var(--accentColor);
    display: flex;
    justify-content: flex-start;
  }

  .split-main-content {
    margin-left: clamp(250px, 24%, 600px);
  }
</style>
