<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import { useFocus } from 'svelte-navigator';
  import Sidebar from '../components/Sidebar.svelte';
  import DataTable from '../components/DataTable.svelte';
  import { notificationMsg, tableNames } from '../stores';
  import {
    NOTIFICATION_TYPE_ERROR,
    BORDER_SIZE,
    MAX_RESIZE_EXPANDABLE_SIZE,
    MIN_RESIZE_EXPANDABLE_SIZE,
  } from '../constants/constants';

  const registerFocus = useFocus();

  // on mousedown for the draggable

  let m_pos;

  function resize(e) {
    const dx = e.x - m_pos;
    m_pos = e.x;
    const leftSidebarContainer = document.getElementById('left-sidebar-conntainer');
    const rightMainContainer = document.getElementById('right-main-content');
    let computedWidth = parseInt(getComputedStyle(leftSidebarContainer, '').width) + dx + 'px';

    if (computedWidth <= MAX_RESIZE_EXPANDABLE_SIZE && computedWidth >= MIN_RESIZE_EXPANDABLE_SIZE) {
      leftSidebarContainer.style.width = computedWidth;
      rightMainContainer.style.width = (Math.round(window.innerHeight / 100) - computedWidth);
      rightMainContainer.style.marginLeft = computedWidth;
    }
  }

  function resizeSideBar(e) {
    e = e.detail.event;
    console.log('Registering mousedown,', e.offsetX);
    if (e.offsetX < BORDER_SIZE) {
      m_pos = e.x;
      document.addEventListener('mousemove', resize, false);
    }
  }

  document.addEventListener(
    'mouseup',
    function () {
      document.removeEventListener('mousemove', resize, false);
    },
    false
  );

  onMount(() => {
    console.log('OnLoad');
    invoke('fetch_tables')
      .then((res) => {
        console.log(res);
        if (res.error_code) {
          notificationMsg.set({ type: NOTIFICATION_TYPE_ERROR, message: res.frontend_msg });
          return;
        }

        if (res.data) {
          if (res.data.rows.length > 0) {
            let tablesResult = [];
            let entityName = '';
            for (const i of res.data.rows[0]) {
              entityName = i.table_catalog;
              tablesResult.push(i.table_name);
            }

            // sort tablenames

            tablesResult = tablesResult.sort((a, b) => a > b);

            tableNames.set({
              tableName: entityName,
              tables: tablesResult,
            });
          }
        }
      })
      .catch((e) => {
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: 'Something went wrong. Check console for more information',
        });
      });
  });
</script>

<div class="main-container" use:registerFocus>
  <div class="columns split-view-container" id="left-sidebar-conntainer">
    <Sidebar on:resizing={resizeSideBar} />
  </div>
  <div class="columns split-main-content" id="right-main-content">
    <DataTable />
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

  .split-view-container {
    position: absolute;
    background-color: var(--accentColor);
    height: 102%;
    width: 24%;
    min-width: 250px;
    max-width: 600px;
  }

  .split-main-content {
    width: 75%;
    margin-left: clamp(250px, 24%, 600px);
  }
</style>
