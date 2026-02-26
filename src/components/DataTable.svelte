<script lang="ts">
  import DataTableToolBar from "./DataTableToolBar.svelte";
  import type { ActiveTable } from "../stores";
  import {
    type ColumnDef,
    getCoreRowModel,
    type CellContext,
  } from "@tanstack/table-core";
  import { createTable } from "@tanstack/svelte-table";
  import CellEditor from "./CellEditor.svelte";
  import { HistoryManager, type HistoryOp } from "../lib/HistoryManager";

  let { activeTableData, fetchData } = $props<{
    activeTableData: ActiveTable;
    fetchData?: (offset: number, limit: number | null) => void | Promise<void>;
  }>();

  let isRefreshing = $state(false);

  async function handleRefresh() {
    isRefreshing = true;
    if (fetchData) await fetchData(offset, limit);
    isRefreshing = false;
  }

  let limit = $state<number | null>(activeTableData?.currentLimit ?? 100);
  let offset = $state(activeTableData?.currentOffset ?? 0);

  function handleLimitChange(newLimit: number | null) {
    limit = newLimit;
    offset = 0; // reset to page 1 on limit change
    if (fetchData) fetchData(offset, limit);
  }

  function gotoNext() {
    if (limit) {
      offset += limit;
      if (fetchData) fetchData(offset, limit);
    }
  }

  function gotoPrev() {
    if (limit) {
      offset = Math.max(0, offset - limit);
      if (fetchData) fetchData(offset, limit);
    }
  }

  function gotoFirst() {
    offset = 0;
    if (fetchData) fetchData(offset, limit);
  }

  function gotoLast() {
    const total = activeTableData?.rowCount || 0;
    if (limit) {
      offset = Math.max(
        0,
        total - (total % limit === 0 ? limit : total % limit),
      );
      if (fetchData) fetchData(offset, limit);
    }
  }

  const historyManager = new HistoryManager();

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

  let editingCell = $state<{ r: number; c: number } | null>(null);
  let editValue = $state("");
  let inputRef = $state<HTMLInputElement | null>(null);
  let contextMenu = $state<{ x: number; y: number; show: boolean } | null>(
    null,
  );

  let sortColumn = $state<string | null>(null);
  let sortDirection = $state<"asc" | "desc" | null>(null);

  $effect(() => {
    // Reset selection when table or page changes
    const _ = activeTableData?.id || activeTableData?.tableName; // re-trigger on table change
    baseSelectedRows = {};
    dragSelectedRows = {};
    baseSelectedCols = {};
    dragSelectedCols = {};
    baseSelectedCells = {};
    dragSelectedCells = {};
    selectionAnchor = null;
    isDragging = false;
    editingCell = null;
    contextMenu = null;
    sortColumn = null;
    sortDirection = null;
    historyManager.clear();
  });

  function commitEdit(newVal: string) {
    if (editingCell) {
      if (activeTableData?.rows && activeTableData.columns) {
        let updatedRows = [...activeTableData.rows];
        let modified = false;
        const ops: HistoryOp[] = [];

        // Apply to all selected elements
        for (let r = 0; r < updatedRows.length; r++) {
          for (let c = 1; c <= activeTableData.columns.length; c++) {
            if (
              selectedRows[r] ||
              selectedCols[c] ||
              selectedCells[`${r}-${c}`]
            ) {
              const oldVal = updatedRows[r][c - 1];
              if (oldVal !== newVal) {
                ops.push({ r, c, oldVal, newVal });
                updatedRows[r][c - 1] = newVal;
                modified = true;
              }
            }
          }
        }

        if (modified) {
          historyManager.push(ops);
          activeTableData.rows = updatedRows;
        }
      }
      editingCell = null;
    }
  }

  function cancelEdit() {
    editingCell = null;
  }

  function handleKeyDown(e: KeyboardEvent) {
    // Escape mapping
    if (e.key === "Escape") {
      baseSelectedRows = {};
      baseSelectedCols = {};
      baseSelectedCells = {};
      selectionAnchor = null;
      editingCell = null;
      return;
    }

    // Undo/Redo intercept
    if (
      (e.ctrlKey || e.metaKey) &&
      (e.key.toLowerCase() === "z" ||
        e.key.toLowerCase() === "r" ||
        e.key.toLowerCase() === "y")
    ) {
      // Don't intercept if editing
      if (editingCell) return;
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA"
      )
        return;

      e.preventDefault();

      if (e.key.toLowerCase() === "z" && !e.shiftKey) {
        if (activeTableData && historyManager.undo(activeTableData.rows)) {
          activeTableData.rows = [...activeTableData.rows];
        }
      } else {
        // Redo (Ctrl+Y, Ctrl+R, or Ctrl+Shift+Z)
        if (activeTableData && historyManager.redo(activeTableData.rows)) {
          activeTableData.rows = [...activeTableData.rows];
        }
      }
      return;
    }
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "a") {
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA"
      )
        return;
      e.preventDefault();
      const allRows: Record<number, boolean> = {};
      for (let i = 0; i < data.length; i++) {
        allRows[i] = true;
      }
      baseSelectedRows = allRows;
      baseSelectedCols = {};
      baseSelectedCells = {};
      return;
    }

    if (e.key === "Delete" || e.key === "Backspace") {
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA"
      )
        return;
      if (editingCell) return;

      const hasSelection =
        Object.keys(selectedCells).length > 0 ||
        Object.keys(selectedRows).length > 0 ||
        Object.keys(selectedCols).length > 0;
      if (!hasSelection) return;

      e.preventDefault();

      if (activeTableData?.rows && activeTableData.columns) {
        let updatedRows = [...activeTableData.rows];
        let modified = false;

        const ops: HistoryOp[] = [];

        for (let r = 0; r < updatedRows.length; r++) {
          for (let c = 1; c <= activeTableData.columns.length; c++) {
            if (
              selectedRows[r] ||
              selectedCols[c] ||
              selectedCells[`${r}-${c}`]
            ) {
              const oldVal = updatedRows[r][c - 1];
              if (oldVal !== "") {
                ops.push({ r, c, oldVal, newVal: "" });
                updatedRows[r][c - 1] = "";
                modified = true;
              }
            }
          }
        }

        if (modified) {
          historyManager.push(ops);
          activeTableData.rows = updatedRows;
        }
      }
    }
  }

  function getSelectedText() {
    const hasCells = Object.keys(selectedCells).length > 0;
    const hasRows = Object.keys(selectedRows).length > 0;
    const hasCols = Object.keys(selectedCols).length > 0;

    if (!hasCells && !hasRows && !hasCols) return null;
    if (!activeTableData?.rows) return null;

    let lines: string[] = [];

    if (hasRows) {
      const sRows = Object.keys(selectedRows)
        .map(Number)
        .sort((a, b) => a - b);
      sRows.forEach((r) => {
        lines.push(activeTableData.rows[r].join("\t"));
      });
    } else if (hasCols) {
      const sCols = Object.keys(selectedCols)
        .map(Number)
        .sort((a, b) => a - b);
      for (let r = 0; r < activeTableData.rows.length; r++) {
        let rowVals: string[] = [];
        sCols.forEach((c) => {
          if (c > 0) rowVals.push(activeTableData.rows[r][c - 1] || "");
        });
        lines.push(rowVals.join("\t"));
      }
    } else {
      const cellKeys = Object.keys(selectedCells);
      let minR = Infinity,
        maxR = -Infinity;
      let minC = Infinity,
        maxC = -Infinity;
      cellKeys.forEach((k) => {
        const [rStr, cStr] = k.split("-");
        const r = parseInt(rStr);
        const c = parseInt(cStr);
        if (r < minR) minR = r;
        if (r > maxR) maxR = r;
        if (c < minC) minC = c;
        if (c > maxC) maxC = c;
      });

      for (let r = minR; r <= maxR; r++) {
        let rowVals: string[] = [];
        for (let c = minC; c <= maxC; c++) {
          if (c > 0) {
            if (selectedCells[`${r}-${c}`]) {
              rowVals.push(activeTableData.rows[r][c - 1] || "");
            } else {
              rowVals.push("");
            }
          }
        }
        if (rowVals.length > 0) lines.push(rowVals.join("\t"));
      }
    }

    return lines.join("\n");
  }

  async function handleCopy(e: ClipboardEvent | undefined = undefined) {
    if (
      document.activeElement?.tagName === "INPUT" ||
      document.activeElement?.tagName === "TEXTAREA"
    )
      return;
    if (editingCell) return;

    const text = getSelectedText();
    if (text !== null) {
      if (e) {
        e.preventDefault();
        e.clipboardData?.setData("text/plain", text);
      } else {
        try {
          await navigator.clipboard.writeText(text);
        } catch (err) {
          console.error("Failed to copy", err);
        }
      }
    }
    contextMenu = null;
  }

  async function handlePaste(e: ClipboardEvent | undefined = undefined) {
    if (
      document.activeElement?.tagName === "INPUT" ||
      document.activeElement?.tagName === "TEXTAREA"
    )
      return;
    if (editingCell) return;

    let text = "";
    if (e) {
      text = e.clipboardData?.getData("text/plain") || "";
      if (text) e.preventDefault();
    } else {
      try {
        text = await navigator.clipboard.readText();
      } catch (err) {
        console.error("Failed to read clipboard", err);
        return;
      }
    }

    if (!text || !activeTableData?.rows) {
      contextMenu = null;
      return;
    }

    const rows = text.split(/\r?\n/).map((row) => row.split("\t"));

    let startR = 0;
    let startC = 1;

    if (selectionAnchor) {
      startR = selectionAnchor.r;
      startC = selectionAnchor.c;
    } else {
      const cellKeys = Object.keys(selectedCells);
      if (cellKeys.length > 0) {
        const [r, c] = cellKeys[0].split("-").map(Number);
        startR = r;
        startC = c;
      } else if (Object.keys(selectedRows).length > 0) {
        startR = Math.min(...Object.keys(selectedRows).map(Number));
        startC = 1;
      } else if (Object.keys(selectedCols).length > 0) {
        startC = Math.min(...Object.keys(selectedCols).map(Number));
        if (startC === 0) startC = 1;
        startR = 0;
      }
    }

    if (startC === 0) startC = 1;

    let updatedRows = [...activeTableData.rows];
    let modified = false;

    let ops: HistoryOp[] = [];

    for (let i = 0; i < rows.length; i++) {
      const r = startR + i;
      if (r >= updatedRows.length) break;

      for (let j = 0; j < rows[i].length; j++) {
        const c = startC + j;
        if (c - 1 < activeTableData.columns.length) {
          const oldVal = updatedRows[r][c - 1];
          const newVal = rows[i][j];
          if (oldVal !== newVal) {
            ops.push({ r, c, oldVal, newVal });
            updatedRows[r][c - 1] = newVal;
            modified = true;
          }
        }
      }
    }

    if (modified) {
      historyManager.push(ops);
      activeTableData.rows = updatedRows;
    }
    contextMenu = null;
  }

  function handleContextMenu(e: MouseEvent, r: number, c: number) {
    if (c === 0) return; // ignore index col
    e.preventDefault();
    contextMenu = {
      x: e.clientX,
      y: e.clientY,
      show: true,
    };

    // Check if right click matches current selection
    let isSelected = false;
    if (selectedRows[r] || selectedCols[c] || selectedCells[`${r}-${c}`]) {
      isSelected = true;
    }

    if (!isSelected) {
      baseSelectedCells = { [`${r}-${c}`]: true };
      baseSelectedRows = {};
      baseSelectedCols = {};
      selectionAnchor = { type: "cell", r, c };
      dragSelectedRows = {};
      dragSelectedCols = {};
      dragSelectedCells = {};
    }
  }

  function handleClickOutside(e: MouseEvent) {
    if (contextMenu) contextMenu = null;

    // Clear selection if clicking outside datatable container
    const target = e.target as HTMLElement;
    if (target && !target.closest(".datatable-main-container")) {
      baseSelectedRows = {};
      baseSelectedCols = {};
      baseSelectedCells = {};
      selectionAnchor = null;
      editingCell = null;
    }
  }

  function handleMouseDown(
    e: MouseEvent,
    type: "row" | "col" | "cell",
    r: number,
    c: number,
  ) {
    if (e.button !== 0) return; // only left click

    if (editingCell) {
      commitEdit(
        document.querySelector(".cell-editor-input")
          ? (document.querySelector(".cell-editor-input") as HTMLInputElement)
              .value
          : "",
      );
    }

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

      if (selectionAnchor && selectionAnchor.type === "cell") {
        editingCell = { r: selectionAnchor.r, c: selectionAnchor.c };
      } else {
        editingCell = null;
      }
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

  // Virtualization state
  let scrollTop = $state(0);
  let containerHeight = $state(800);
  const ROW_HEIGHT = 32;

  let totalRows = $derived(table.getRowModel().rows.length);
  let startIndex = $derived(
    Math.max(0, Math.floor(scrollTop / ROW_HEIGHT) - 15),
  );
  let endIndex = $derived(
    Math.min(
      totalRows,
      Math.ceil((scrollTop + containerHeight) / ROW_HEIGHT) + 15,
    ),
  );
  let visibleRows = $derived(
    table.getRowModel().rows.slice(startIndex, endIndex),
  );
</script>

<svelte:window
  onmouseup={handleMouseUp}
  onkeydown={handleKeyDown}
  oncopy={handleCopy}
  onpaste={handlePaste}
  onclick={handleClickOutside}
/>

<div class="datatable-main-container">
  {#if activeTableData && activeTableData.tableName}
    <DataTableToolBar
      {limit}
      onLimitChange={handleLimitChange}
      currentPage={activeTableData.currentPage}
      maxPage={activeTableData.maxPage}
      rowCount={activeTableData.rowCount}
      {gotoNext}
      {gotoPrev}
      {gotoFirst}
      {gotoLast}
      {isRefreshing}
      onRefresh={handleRefresh}
    />

    <div
      class="table-scroll-container"
      onscroll={(e) => (scrollTop = e.currentTarget.scrollTop)}
      bind:clientHeight={containerHeight}
    >
      <table>
        <thead>
          {#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
            <tr>
              {#each headerGroup.headers as header, c (header.id)}
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
                    <div class="header-content">
                      <span class="header-text"
                        >{header.column.columnDef.header}</span
                      >
                      {#if header.column.id !== "index"}
                        <button
                          class="sort-btn"
                          onmousedown={(e) => {
                            e.stopPropagation();
                          }}
                          onclick={(e) => {
                            e.stopPropagation();
                            e.preventDefault();
                            if (sortColumn === header.column.id) {
                              if (sortDirection === "asc") {
                                sortDirection = "desc";
                              } else if (sortDirection === "desc") {
                                sortColumn = null;
                                sortDirection = null;
                              }
                            } else {
                              sortColumn = header.column.id;
                              sortDirection = "asc";
                            }
                            console.log(
                              `Backend sort triggered for column: ${sortColumn || "none"}, direction: ${sortDirection || "none"}`,
                            );
                          }}
                          aria-label="Sort {header.column.id}"
                        >
                          {#if sortColumn === header.column.id && sortDirection === "asc"}
                            <svg
                              xmlns="http://www.w3.org/2000/svg"
                              width="14"
                              height="14"
                              viewBox="0 0 24 24"
                              fill="none"
                              stroke="currentColor"
                              stroke-width="2"
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              class="lucide lucide-arrow-up"
                            >
                              <path d="m5 12 7-7 7 7" />
                              <path d="M12 19V5" />
                            </svg>
                          {:else if sortColumn === header.column.id && sortDirection === "desc"}
                            <svg
                              xmlns="http://www.w3.org/2000/svg"
                              width="14"
                              height="14"
                              viewBox="0 0 24 24"
                              fill="none"
                              stroke="currentColor"
                              stroke-width="2"
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              class="lucide lucide-arrow-down"
                            >
                              <path d="M12 5v14" />
                              <path d="m19 12-7 7-7-7" />
                            </svg>
                          {:else}
                            <svg
                              xmlns="http://www.w3.org/2000/svg"
                              width="14"
                              height="14"
                              viewBox="0 0 24 24"
                              fill="none"
                              stroke="currentColor"
                              stroke-width="2"
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              class="lucide lucide-arrow-up-down"
                            >
                              <path d="m21 16-4 4-4-4" />
                              <path d="M17 20V4" />
                              <path d="m3 8 4-4 4 4" />
                              <path d="M7 4v16" />
                            </svg>
                          {/if}
                        </button>
                      {/if}
                    </div>
                  {/if}
                </th>
              {/each}
            </tr>
          {/each}
        </thead>
        <tbody>
          {#if startIndex > 0}
            <tr style="height: {startIndex * ROW_HEIGHT}px">
              <td colspan={1000} style="padding: 0; border: none;"></td>
            </tr>
          {/if}
          {#each visibleRows as row (row.id)}
            {@const r = row.index}
            <tr class:selected-row={selectedRows[r]}>
              {#each row.getVisibleCells() as cell, c (cell.id)}
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
                  ondblclick={(e) => {
                    e.stopPropagation();
                    editingCell = { r, c };
                  }}
                  oncontextmenu={(e) => handleContextMenu(e, r, c)}
                >
                  <span
                    style="visibility: {editingCell?.r === r &&
                    editingCell?.c === c
                      ? 'hidden'
                      : 'visible'}"
                  >
                    {cell.getValue()}
                  </span>
                  {#if editingCell?.r === r && editingCell?.c === c}
                    <CellEditor
                      initialValue={cell.getValue() as string}
                      onCommit={commitEdit}
                      onCancel={cancelEdit}
                    />
                  {/if}
                </td>
              {/each}
            </tr>
          {/each}
          {#if endIndex < totalRows}
            <tr style="height: {(totalRows - endIndex) * ROW_HEIGHT}px">
              <td colspan={1000} style="padding: 0; border: none;"></td>
            </tr>
          {/if}
        </tbody>
      </table>
    </div>
  {:else}
    <div class="empty-state">
      <p>No data selected. Click a table in the sidebar to view data.</p>
    </div>
  {/if}

  {#if contextMenu && contextMenu.show}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="context-menu"
      style={`top: ${contextMenu.y}px; left: ${contextMenu.x}px`}
      onclick={(e) => e.stopPropagation()}
    >
      <div class="menu-item" onclick={() => handleCopy()}>
        Copy
        <span class="shortcut">Ctrl C</span>
      </div>
      <div class="menu-item" onclick={() => handlePaste()}>
        Paste
        <span class="shortcut">Ctrl V</span>
      </div>
    </div>
  {/if}

  {#if activeTableData && activeTableData.tableName}
    <div class="datatable-footer">
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span
        class="pagination-info"
        onclick={() => {
          if (fetchData) fetchData(offset, limit);
        }}
        title="Click to refresh data"
      >
        <i class="fa-solid fa-rotate-right refresh-icon"></i>
        {#if limit === null}
          All rows of {activeTableData.rowCount || 0}
        {:else}
          {offset + 1}-{Math.min(
            offset + (limit || 100),
            activeTableData.rowCount || 0,
          )} of {activeTableData.rowCount || 0}
        {/if}
      </span>
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
  }

  .datatable-footer {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    padding: 4px 12px;
    background-color: #f3f4f6; /* Subtle grey background */
    border-top: 1px solid #d1d5db;
    height: 30px;
    font-size: 12px;
    font-weight: 500;
    color: #4b5563;
    font-family: inherit;
    flex-shrink: 0;
  }

  .pagination-info {
    cursor: pointer;
    transition: color 0.15s;
    user-select: none;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .pagination-info:hover {
    color: #111827; /* Darken on hover */
  }

  .refresh-icon {
    font-size: 11px;
    color: #6b7280;
  }

  .pagination-info:hover .refresh-icon {
    color: #3b82f6; /* Highlight refresh icon on hover */
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
    position: relative;
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

  .header-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    gap: 8px;
  }

  .header-text {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .sort-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 2px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #4b5563; /* grey-600 */
    transition:
      background-color 0.2s,
      color 0.2s;
  }

  .sort-btn:hover {
    background-color: #d1d5db; /* grey-300 */
    color: #111827; /* grey-900 */
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

  .context-menu {
    position: fixed;
    background: white;
    border: 1px solid #d1d5db;
    border-radius: 4px;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
    padding: 4px 0;
    z-index: 50;
    min-width: 150px;
  }

  .menu-item {
    padding: 6px 12px;
    cursor: pointer;
    font-size: 13px;
    color: #374151;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .menu-item:hover {
    background-color: #f3f4f6;
  }

  .menu-item .shortcut {
    color: #9ca3af;
    font-size: 12px;
  }
</style>
