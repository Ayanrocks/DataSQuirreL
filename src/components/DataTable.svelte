<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import DataTableToolBar from './DataTableToolBar.svelte';
  import { activeTable, notificationMsg } from '../stores';
  import type { ActiveTable } from '../stores';
  import { invoke } from '@tauri-apps/api/core';
  import { NOTIFICATION_TYPE_ERROR, PAGINATION_SIZE } from '../constants/constants';
  import {
    createTable,
    type ColumnDef,
    type Table,
    type Updater,
    getCoreRowModel,
    type CellContext,
  } from '@tanstack/table-core';

  let table: Table<any>;
  let columns: ColumnDef<any>[] = [];
  let data: any[] = [];
  let rowCount = 0;


  
</script>

<div class="datatable-main-container">
  
</div>

<style>
  .datatable-main-container {
    margin-top: 10px;
    width: 100%;
    height: 100%;
    padding: 10px;
  }

  .datagrid-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 90%;
    position: relative;
    margin: 55px auto 10px auto;
  }

  .table-container {
    margin-top: 5%;
    height: 90%;
    width: 100%;
    overflow: auto; /* Add scroll for large tables */
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 8px;
    text-align: left;
  }

  th {
    background-color: var(--accentColor);
    color: white;
  }
</style>
