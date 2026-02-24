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

  let baseSelectedRows = $state<Record<number, boolean>>({});
  let dragSelectedRows = $state<Record<number, boolean>>({});
  let selectedRows = $derived({ ...baseSelectedRows, ...dragSelectedRows });

  let baseSelectedCols = $state<Record<number, boolean>>({});
  let dragSelectedCols = $state<Record<number, boolean>>({});
  let selectedCols = $derived({ ...baseSelectedCols, ...dragSelectedCols });

  let baseSelectedCells = $state<Record<string, boolean>>({});
  let dragSelectedCells = $state<Record<string, boolean>>({});
  let selectedCells = $derived({ ...baseSelectedCells, ...dragSelectedCells });

  let selectionAnchor = $state<{
    type: "row" | "col" | "cell";
    r: number;
    c: number;
  } | null>(null);
  let isDragging = $state(false);

  $effect(() => {
    // Reset selection when table or page changes
    const _ = activeTableData;
    baseSelectedRows = {};
    dragSelectedRows = {};
    baseSelectedCols = {};
    dragSelectedCols = {};
    baseSelectedCells = {};
    dragSelectedCells = {};
    selectionAnchor = null;
    isDragging = false;
  });

  function handleMouseDown(
    e: MouseEvent,
    type: "row" | "col" | "cell",
    r: number,
    c: number,
  ) {
    if (e.button !== 0) return; // only left click
    e.preventDefault();
    isDragging = true;

    if (e.shiftKey && selectionAnchor && selectionAnchor.type === type) {
      if (!(e.metaKey || e.ctrlKey)) {
        baseSelectedRows = {};
        baseSelectedCols = {};
        baseSelectedCells = {};
      }
      updateDragSelection(selectionAnchor, { type, r, c });
    } else {
      selectionAnchor = { type, r, c };
      if (e.metaKey || e.ctrlKey) {
        if (type === "row") {
          const newRows = { ...baseSelectedRows };
          if (newRows[r]) delete newRows[r];
          else newRows[r] = true;
          baseSelectedRows = newRows;
        } else if (type === "col") {
          const newCols = { ...baseSelectedCols };
          if (newCols[c]) delete newCols[c];
          else newCols[c] = true;
          baseSelectedCols = newCols;
        } else {
          const id = `${r}-${c}`;
          const newCells = { ...baseSelectedCells };
          if (newCells[id]) delete newCells[id];
          else newCells[id] = true;
          baseSelectedCells = newCells;
        }
      } else {
        baseSelectedRows = type === "row" ? { [r]: true } : {};
        baseSelectedCols = type === "col" ? { [c]: true } : {};
        baseSelectedCells = type === "cell" ? { [`${r}-${c}`]: true } : {};
      }
      dragSelectedRows = {};
      dragSelectedCols = {};
      dragSelectedCells = {};
    }
  }

  function handleMouseEnter(
    e: MouseEvent,
    type: "row" | "col" | "cell",
    r: number,
    c: number,
  ) {
    if (!isDragging || !selectionAnchor) return;
    updateDragSelection(selectionAnchor, { type, r, c });
  }

  function handleMouseUp() {
    if (isDragging) {
      isDragging = false;
      baseSelectedRows = { ...baseSelectedRows, ...dragSelectedRows };
      baseSelectedCols = { ...baseSelectedCols, ...dragSelectedCols };
      baseSelectedCells = { ...baseSelectedCells, ...dragSelectedCells };
      dragSelectedRows = {};
      dragSelectedCols = {};
      dragSelectedCells = {};
    }
  }

  function updateDragSelection(
    anchor: { type: "row" | "col" | "cell"; r: number; c: number },
    current: { type: "row" | "col" | "cell"; r: number; c: number },
  ) {
    const newDragRows: Record<number, boolean> = {};
    const newDragCols: Record<number, boolean> = {};
    const newDragCells: Record<string, boolean> = {};

    if (anchor.type === "row" && current.type === "row") {
      const minR = Math.min(anchor.r, current.r);
      const maxR = Math.max(anchor.r, current.r);
      for (let i = minR; i <= maxR; i++) newDragRows[i] = true;
    } else if (anchor.type === "col" && current.type === "col") {
      const minC = Math.min(anchor.c, current.c);
      const maxC = Math.max(anchor.c, current.c);
      for (let i = minC; i <= maxC; i++) newDragCols[i] = true;
    } else if (anchor.type === "cell" && current.type === "cell") {
      const minR = Math.min(anchor.r, current.r);
      const maxR = Math.max(anchor.r, current.r);
      const minC = Math.min(anchor.c, current.c);
      const maxC = Math.max(anchor.c, current.c);
      for (let i = minR; i <= maxR; i++) {
        for (let j = minC; j <= maxC; j++) {
          newDragCells[`${i}-${j}`] = true;
        }
      }
    }
    dragSelectedRows = newDragRows;
    dragSelectedCols = newDragCols;
    dragSelectedCells = newDragCells;
  }

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

<svelte:window onmouseup={handleMouseUp} />

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
              {#each headerGroup.headers as header, c (header.id)}
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <th
                  colSpan={header.colSpan}
                  class:selected-col={selectedCols[c]}
                  onmousedown={(e) => {
                    if (header.column.id === "index") return;
                    handleMouseDown(e, "col", 0, c);
                  }}
                  onmouseenter={(e) => {
                    if (header.column.id === "index") return;
                    handleMouseEnter(e, "col", 0, c);
                  }}
                  style:cursor={header.column.id === "index"
                    ? "default"
                    : "pointer"}
                >
                  {#if !header.isPlaceholder}
                    {header.column.columnDef.header}
                  {/if}
                </th>
              {/each}
            </tr>
          {/each}
        </thead>
        <tbody>
          {#each table.getRowModel().rows as row, r (row.id)}
            <tr class:selected-row={selectedRows[r]}>
              {#each row.getVisibleCells() as cell, c (cell.id)}
                <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                <td
                  class:selected-cell={selectedCells[`${r}-${c}`] ||
                    selectedCols[c]}
                  onmousedown={(e) => {
                    if (cell.column.id === "index") {
                      handleMouseDown(e, "row", r, c);
                    } else {
                      handleMouseDown(e, "cell", r, c);
                    }
                  }}
                  onmouseenter={(e) => {
                    if (cell.column.id === "index") {
                      handleMouseEnter(e, "row", r, c);
                    } else {
                      handleMouseEnter(e, "cell", r, c);
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
    overscroll-behavior: contain;
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
    font-family: "JetBrains Mono", monospace;
  }

  td {
    cursor: cell;
    user-select: none; /* prevent native text selection when dragging */
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
    user-select: none; /* prevent text selection when dragging columns */
  }

  th.selected-col {
    background-color: #bae6fd !important;
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
