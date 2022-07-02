<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import { useFocus } from 'svelte-navigator';
  import { notificationMsg, tableNames } from '../stores';
  import { NOTIFICATION_TYPE_SUCCESS, NOTIFICATION_TYPE_ERROR } from '../constants/constants';

  const registerFocus = useFocus();
  let sideBarColumn = 'Table Names';
  let tables = [];

  const BORDER_SIZE = 4;

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

  tableNames.subscribe((e) => {
    tables = e.tables;
    sideBarColumn = e.tableName;
  });

  // on mousedown for the draggable
  let m_pos;

  function resize(e) {
    const dx = e.x - m_pos;
    m_pos = e.x;
    const panel = document.getElementById('left-sidebar');
    panel.style.width = parseInt(getComputedStyle(panel, '').width) + dx + 'px';
  }

  function resizeSideBar(e) {
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
</script>

<div class="main-container" use:registerFocus>
  <div class="columns split-view-container">
    <div class="column is-one-quarter split-sidebar" id="left-sidebar">
      <div class="split-sidebar-draggable-div" on:mousedown={resizeSideBar} />
      <div class="table-container has-text-centered">
        <table class="table is-fullwidth is-hoverable has-text-centered">
          <thead>
            <tr>
              <th>{sideBarColumn}</th>
            </tr>
          </thead>
          <tbody>
            {#each tables as t}
              <tr>
                <th>{t}</th>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>
    <div class="column split-main-content">Auto</div>
  </div>
</div>

<style>
  .main-container {
    height: 100vh;
    width: 100vw;
  }

  .split-view-container {
    height: 102%;
    width: 100%;
  }

  .split-sidebar {
    position: absolute;
    width: 96px;
    height: 102%;
    left: 0;
    background-color: #f0f0ff;
    min-width: 250px;
    max-width: 600px;
  }

  .split-sidebar-draggable-div {
    content: '';
    background-color: rgb(161, 161, 161);
    position: absolute;
    right: 0;
    width: 4px;
    height: 98%;
    cursor: ew-resize;
  }

  .table-container {
    overflow-y: scroll;
  }
</style>
