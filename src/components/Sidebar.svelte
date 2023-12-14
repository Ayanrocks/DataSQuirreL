<script>
  import { activeTable, notificationMsg, tableNames } from '../stores';
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { NOTIFICATION_TYPE_ERROR, PAGINATION_SIZE } from '../constants/constants';

  const dispatch = createEventDispatcher();
  let activeTableName = '';

  function resize(e) {
    dispatch('resizing', {
      event: e,
    });
  }

  let sideBarColumn = 'Table Names';
  let tables = [];

  tableNames.subscribe((e) => {
    tables = e.tables;
    sideBarColumn = e.tableName;
  });

  activeTable.subscribe((val) => {
    activeTableName = val.tableName;
  });

  function clickedSidebar(e) {
    // checking if the same table is opened or not
    if (activeTableName == e) {
      // no change needed
      return;
    }
    // Invoke the command
    invoke('fetch_table_data', {
      reqPayload: {
        table_name: e,
      },
    })
      .then((res) => {
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
        activeTableData.maxPage = parseInt(res.data.row_count / PAGINATION_SIZE);
        if (res.data.row_count % PAGINATION_SIZE) {
            activeTableData.maxPage++;
        }

        activeTable.set(activeTableData);
      })
      .catch((e) => {
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: e,
        });
      });
  }
</script>

<div class="column is-one-quarter split-sidebar" id="left-sidebar">
  <div class="split-sidebar-draggable-div" on:mousedown={resize}>
    <span id="resize-icon">
      <i class="fas fa-solid fa-grip-lines-vertical" />
    </span>
  </div>
  <div class="sidebar-content">
    <div class="db-selector-dropdown">
      <div class="control has-icons-left">
        <div class="select is-rounded">
          <select>
            <option selected>{sideBarColumn}</option>
          </select>
        </div>
        <div class="dropdown-icon-wrapper">
          <span class="icon is-left">
            <i class="fas fa-solid fa-database"></i>
          </span>
        </div>
      </div>
    </div>
    <div class="table-list has-text-left">
      <h1>Tables</h1>
      <ul class="table-list-ul">
        {#each tables as t (t)}
          <li class="rounded-rectangle" on:click={clickedSidebar(t)} on:keydown={clickedSidebar(t)}>
            <span class="icon is-left">
              <i class="fas fa-thin fa-table"></i>
            </span>
            {t}
          </li>
        {/each}
      </ul>
    </div>
  </div>
</div>

<style>
  .split-sidebar {
    height: 102%;
    width: 100%;
    flex: none;
    color: var(--offWhite);
    background-color: var(--primaryColor);
  }

  .sidebar-content {
    display: flex;
    flex-direction: column;
    padding: 20px 0 20px 5px;
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

  .db-selector-dropdown {
    /* text-align: center; */
    margin: 0 auto;
  }

  div.select select {
    color: var(--offWhite);
    background-color: var(--secondaryColor);
  }

  div.select::after {
    border-color: var(--primaryColor);
  }

  .table-list {
    overflow-y: scroll;
    overflow-x: hidden;
    margin: 20px 5px;
    line-height: 25px;
    word-break: break-all;
  }

  .rounded-rectangle {
    border-radius: 7px;
  }

  .table-list-ul li {
    cursor: pointer;
    transition: cubic-bezier(0.95, 0.05, 0.795, 0.035);
    padding: 1px;
  }

  .table-list-ul li:hover {
    background-color: var(--secondaryColor);
  }

  .fa-table {
    color: var(--accentColor);
  }

  .fa-database {
    color: var(--offBlue);
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
