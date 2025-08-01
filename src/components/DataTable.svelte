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
  } from '@tanstack/svelte-table';

  let table: Table<any>;
  let columns: ColumnDef<any>[] = [];
  let data: any[] = [];
  let rowCount = 0;

  let tableData: ActiveTable = {
    tableName: '',
    rows: [],
    columns: [],
    rowCount: 0,
    currentPage: 0,
    maxPage: 0,
  };

  activeTable.subscribe((val: ActiveTable) => {
    console.log('ActiveTable Subscribe', val);
    tableData = val;
  });

  onMount(() => {
    columns = [
      {
        header: '#',
        accessorFn: (_row, index) => index + 1,
        cell: (info: CellContext<any, unknown>) => info.getValue(),
        enableSorting: true,
      },
      ...tableData.columns.map((colName) => ({
        accessorKey: colName,
        header: colName,
        cell: (info: CellContext<any, unknown>) => info.getValue(),
        enableSorting: true,
      })),
    ];

    data = tableData.rows.map((row) => {
      let singleRowData: any = {};
      tableData.columns.forEach((colName, index) => {
        singleRowData[colName] = row[index];
      });
      return singleRowData;
    });

    table = createTable({
      data,
      columns,
      getCoreRowModel: getCoreRowModel(),
      state: {},
      onStateChange: (updater: Updater<any>) => {
        // Handle state changes if needed
      },
      renderFallbackValue: null,
      debugTable: true,
      debugHeaders: true,
      debugColumns: true,
    });
  });

  $: if (tableData.tableName !== '' && tableData.columns.length !== 0 && table) {
    rowCount = tableData.rowCount;

    columns = [
      {
        header: '#',
        accessorFn: (_row, index) => index + 1,
        cell: (info: CellContext<any, unknown>) => info.getValue(),
        enableSorting: true,
      },
      ...tableData.columns.map((colName) => ({
        accessorKey: colName,
        header: colName,
        cell: (info: CellContext<any, unknown>) => info.getValue(),
        enableSorting: true,
      })),
    ];

    data = tableData.rows.map((row) => {
      let singleRowData: any = {};
      tableData.columns.forEach((colName, index) => {
        singleRowData[colName] = row[index];
      });
      return singleRowData;
    });

    table.setOptions((prev) => ({
      ...prev,
      columns,
      data,
    }));
  }

  let gotoNext = () => {
    let offsetVal = PAGINATION_SIZE * tableData.currentPage;
    fetchNextRecordBatch(tableData.tableName, offsetVal);
  };

  let gotoPrev = () => {
    // Need to implement logic to fetch previous batch if needed, or rely on already loaded data
    // For now, this is a placeholder.
  };

  function fetchNextRecordBatch(tableName: string, offsetVal: number) {
    console.log('TABLENAME: ', tableName);
    invoke<{ data: { rows: any[][], currentPage: number } }>('fetch_table_data_with_offset', {
      reqPayload: {
        table_name: tableName,
        offset: offsetVal,
      },
    })
      .then((res) => {
        console.log(res);
        let data = res.data;

        tableData.rows = [...data.rows];
        tableData.currentPage = data.currentPage;

        activeTable.set(tableData);
      })
      .catch((e: string) => {
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: e,
        });
      });
  }

  onDestroy(() => {
    // TanStack Table does not require explicit destruction like AG Grid
  });
</script>

<div class="datatable-main-container">
  <div class="datagrid-container">
    <DataTableToolBar currentPage={tableData.currentPage} maxPage={tableData.maxPage} {gotoNext} {gotoPrev} />

    {#if table}
      <div class="table-container">
        <table>
          <thead>
            {#each table.getHeaderGroups() as headerGroup}
              <tr>
                {#each headerGroup.headers as header}
                  <th colSpan={header.colSpan}>
                    {#if !header.isPlaceholder}
                      {header.isPlaceholder ? null : header.column.columnDef.header}
                    {/if}
                  </th>
                {/each}
              </tr>
            {/each}
          </thead>
          <tbody>
            {#each table.getRowModel().rows as row}
              <tr>
                {#each row.getVisibleCells() as cell}
                  <td>
                    {cell.getIsGrouped() ? null : cell.getValue()}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
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
