<script lang="ts">
  import DataTableToolBar from "./DataTableToolBar.svelte";
  import type { ActiveTable } from "../stores";
  import {
    type ColumnDef,
    getCoreRowModel,
    type CellContext,
  } from "@tanstack/table-core";
  import { createTable } from "@tanstack/svelte-table";

  let { activeTableData } = $props<{ activeTableData: ActiveTable }>();

  let selectedRowId = $state<string | null>(null);
  let selectedCellId = $state<string | null>(null);

  $effect(() => {
    // Reset selection when table or page changes
    const _ = activeTableData;
    selectedRowId = null;
    selectedCellId = null;
  });

  let columns = $derived.by(() => {
    let cols: ColumnDef<any>[] = [
      {
        id: "index",
        header: "#",
        accessorFn: (_row, index) =>
          index + 1 + ((activeTableData?.currentPage || 1) - 1) * 100,
        size: 50,
      },
    ];

    if (activeTableData?.columns) {
      activeTableData.columns.forEach((col: string[]) => {
        let colName = col[1];
        cols.push({
          accessorKey: colName,
          header: colName,
          cell: (info: CellContext<any, any>) => info.getValue(),
        });
      });
    }
    return cols;
  });

  let data = $derived.by(() => {
    if (!activeTableData?.rows || !activeTableData?.columns) return [];
    return activeTableData.rows.map((row: string[]) => {
      let rowObj: { [key: string]: string } = {};
      activeTableData.columns.forEach((col: string[], index: number) => {
        let colName = col[1];
        rowObj[colName] = row[index];
      });
      return rowObj;
    });
  });

  let table = createTable({
    get data() {
      return data;
    },
    get columns() {
      return columns;
    },
    getCoreRowModel: getCoreRowModel(),
    renderFallbackValue: null,
  });
</script>

<div class="datatable-main-container">
  {#if activeTableData && activeTableData.tableName}
    <DataTableToolBar
      currentPage={activeTableData.currentPage}
      maxPage={activeTableData.maxPage}
      rowCount={activeTableData.rowCount}
      gotoNext={() => {
        console.log("Next page requested");
      }}
      gotoPrev={() => {
        console.log("Prev page requested");
      }}
    />

    <div class="table-scroll-container">
      <table>
        <thead>
          {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
            <tr>
              {#each headerGroup.headers as header (header.id)}
                <th colSpan={header.colSpan}>
                  {#if !header.isPlaceholder}
                    {header.column.columnDef.header}
                  {/if}
                </th>
              {/each}
            </tr>
          {/each}
        </thead>
        <tbody>
          {#each table.getRowModel().rows as row (row.id)}
            <tr class:selected-row={selectedRowId === row.id}>
              {#each row.getVisibleCells() as cell (cell.id)}
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <td
                  class:selected-cell={selectedCellId === cell.id &&
                    selectedRowId !== row.id}
                  onclick={() => {
                    if (cell.column.id === "index") {
                      selectedRowId = selectedRowId === row.id ? null : row.id;
                      selectedCellId = null;
                    } else {
                      selectedCellId = cell.id;
                      selectedRowId = null;
                    }
                  }}
                >
                  {cell.getValue()}
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="empty-state">
      <p>No data selected. Click a table in the sidebar to view data.</p>
    </div>
  {/if}
</div>

<style>
  .datatable-main-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: #ffffff;
    overflow: hidden;
  }

  .table-scroll-container {
    flex: 1 1 0;
    min-height: 0;
    min-width: 0;
    overflow: auto;
    width: 100%;
    border-bottom: 1px solid #d1d5db;
    margin-bottom: 24px;
  }

  table {
    width: 100%;
    border-collapse: separate;
    border-spacing: 0;
    font-size: 13px;
    font-family: "JetBrains Mono", monospace;
    color: #1f2937;
  }

  th,
  td {
    border-right: 1px solid #d1d5db;
    border-bottom: 1px solid #d1d5db;
    padding: 6px 12px;
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 300px;
  }

  td {
    cursor: cell;
  }

  th {
    background-color: #9ca3af; /* Header matching the screenshot grey */
    color: #111827;
    font-weight: 600;
    position: sticky;
    top: 0;
    z-index: 5;
    border-top: none;
    box-shadow: 0 1px 0 #d1d5db; /* bottom border representation for sticky */
    border-bottom: none; /* we use box-shadow to prevent scroll bug on sticky */
  }

  /* Index column header */
  th:first-child,
  td:first-child {
    text-align: center;
    width: 50px;
    min-width: 50px;
    background-color: #9ca3af;
    color: #111827;
    position: sticky;
    left: 0;
    z-index: 6; /* Higher z-index for corner cell */
    font-weight: 600;
  }

  td:first-child {
    background-color: #f3f4f6; /* Lighter grey for index column data */
    z-index: 4; /* Higher than normal cells but below headers */
    box-shadow: 1px 0 0 #d1d5db;
    border-right: none;
    cursor: pointer;
  }

  tbody tr:nth-child(even) td:not(:first-child) {
    background-color: #f9fafb;
  }

  tbody tr:hover td:not(:first-child) {
    background-color: #f3f4f6;
  }

  tbody tr.selected-row td {
    background-color: #e0f2fe !important; /* light blue for selected row */
    box-shadow:
      inset 0 2px 0 0 #3b82f6,
      inset 0 -2px 0 0 #3b82f6;
  }

  tbody tr.selected-row td:first-child {
    background-color: #bae6fd !important; /* slightly darker for the index */
    box-shadow:
      inset 0 2px 0 0 #3b82f6,
      inset 0 -2px 0 0 #3b82f6,
      inset 2px 0 0 0 #3b82f6;
  }

  tbody tr.selected-row td:last-child {
    box-shadow:
      inset 0 2px 0 0 #3b82f6,
      inset 0 -2px 0 0 #3b82f6,
      inset -2px 0 0 0 #3b82f6;
  }

  td.selected-cell {
    outline: 2px solid #3b82f6; /* blue outline */
    outline-offset: -2px;
    background-color: #e0f2fe !important;
  }

  .empty-state {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    color: #6b7280;
    font-size: 14px;
  }
</style>
