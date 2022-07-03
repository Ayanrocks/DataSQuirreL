<script>
  import { tableNames } from '../stores';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

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
</script>

<div class="column is-one-quarter split-sidebar" id="left-sidebar">
  <div class="split-sidebar-draggable-div" on:mousedown={resize} />
  <div class="sidebar-content">
    <div class="db-selector-dropdown">
      <div class="control has-icons-left">
        <div class="select">
          <select class="rounded-rectangle">
            <option selected>{sideBarColumn}</option>
          </select>
        </div>
        <div class="dropdown-icon-wrapper">
          <span class="icon is-left">
            <i class="fas fa-solid fa-database" />
          </span>
        </div>
      </div>
    </div>
    <div class="table-list has-text-left">
      <h1>Tables</h1>
      <ul class="table-list-ul">
        {#each tables as t}
          <li class="rounded-rectangle">
            <span class="icon is-left">
              <i class="fas fa-thin fa-table" />
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
    background-color: rgb(191, 191, 191);
    width: 4px;
    height: 100%;
    margin-left: 30px;
    cursor: ew-resize;
  }

  .db-selector-dropdown {
    /* text-align: center; */
    margin: 0 auto;
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
    background-color: #c5c5c5;
  }

  .fa-table {
    color: #3ab0ff;
  }

  .fa-database {
    color: #f87474;
  }
</style>
