<script lang="ts">
  import DataTableToolBar from "./DataTableToolBar.svelte";
  import QueryPreviewModal from "./QueryPreviewModal.svelte";
  import ExportModal from "./ExportModal.svelte";
  import Loader from "./Loader.svelte";
  import type { ActiveTable } from "../stores";
  import {
    type ColumnDef,
    getCoreRowModel,
    type CellContext,
  } from "@tanstack/table-core";
  import { createTable } from "@tanstack/svelte-table";
  import CellEditor from "./CellEditor.svelte";
  import { HistoryManager, type HistoryOp } from "../lib/HistoryManager";
  import { isShortcut, Shortcuts } from "../lib/shortcuts";
  import {
    TransactionManager,
    type TransactionChange,
  } from "../lib/TransactionManager";
  import { invoke } from "@tauri-apps/api/core";
  import { confirm } from "@tauri-apps/plugin-dialog";
  import { notificationMsg } from "../stores";
  import {
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_ERROR,
    INVOKE_COMMIT_TRANSACTION,
  } from "../constants/constants";

  let { activeTableData = $bindable(), fetchData } = $props<{
    activeTableData: ActiveTable;
    fetchData?: (
      offset: number,
      limit: number | null,
      sortColumn?: string | null,
      sortDirection?: string | null,
      whereClause?: string | null,
    ) => void | Promise<void>;
  }>();

  let isRefreshing = $state(false);

  async function handleRefresh() {
    if (transactionManager.hasChanges()) {
      const doDiscard = await confirm(
        "You have uncommitted changes. Are you sure you want to discard them?",
        { title: "Discard Changes", kind: "warning" },
      );
      if (!doDiscard) {
        return;
      }
      transactionManager.clear();
      transactionChangesMap = new Map();
    }
    isRefreshing = true;
    if (fetchData)
      await fetchData(offset, limit, sortColumn, sortDirection, whereClause);
    isRefreshing = false;
  }

  async function handleRevert() {
    console.log("Revert clicked! hasChanges:", transactionManager.hasChanges());
    if (transactionManager.hasChanges()) {
      let start = Date.now();
      let doRevert = await confirm(
        "Are you sure you want to revert all local changes?",
        { title: "Revert Changes", kind: "warning" },
      );

      if (!doRevert && Date.now() - start < 200) {
        doRevert = window.confirm(
          "Are you sure you want to revert all local changes?",
        );
      }

      console.log("Coming Here for revert", doRevert);

      if (!doRevert) return;

      if (activeTableData?.rows) {
        let updatedRows = [...activeTableData.rows];
        while (historyManager.undo(updatedRows)) {}
        // Svelte 5: replace entire object to safely trigger bindable update
        activeTableData = { ...activeTableData, rows: updatedRows };
      }

      transactionManager.clear();
      transactionChangesMap = new Map();
      historyManager.clear();
    }
  }

  let limit = $state<number | null>(activeTableData?.currentLimit ?? 100);
  let offset = $state(activeTableData?.currentOffset ?? 0);

  let showExportModal = $state(false);

  function handleExport() {
    showExportModal = true;
  }

  function handleLimitChange(newLimit: number | null) {
    limit = newLimit;
    offset = 0; // reset to page 1 on limit change
    if (fetchData)
      fetchData(offset, limit, sortColumn, sortDirection, whereClause);
  }

  function gotoNext() {
    if (limit) {
      offset += limit;
      if (fetchData)
        fetchData(offset, limit, sortColumn, sortDirection, whereClause);
    }
  }

  function gotoPrev() {
    if (limit) {
      offset = Math.max(0, offset - limit);
      if (fetchData)
        fetchData(offset, limit, sortColumn, sortDirection, whereClause);
    }
  }

  function gotoFirst() {
    offset = 0;
    if (fetchData)
      fetchData(offset, limit, sortColumn, sortDirection, whereClause);
  }

  function gotoLast() {
    const total = activeTableData?.rowCount || 0;
    if (limit) {
      offset = Math.max(
        0,
        total - (total % limit === 0 ? limit : total % limit),
      );
      if (fetchData)
        fetchData(offset, limit, sortColumn, sortDirection, whereClause);
    }
  }

  const historyManager = new HistoryManager();
  const transactionManager = new TransactionManager();

  // Make changes reactive so UI updates
  let transactionChangesMap = $state(new Map<number, TransactionChange>());

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

  let hasSelection = $derived(
    Object.keys(selectedRows).length > 0 ||
      Object.keys(selectedCols).length > 0 ||
      Object.keys(selectedCells).length > 0,
  );

  let editingCell = $state<{ r: number; c: number } | null>(null);
  let editValue = $state("");
  let inputRef = $state<HTMLInputElement | null>(null);
  let contextMenu = $state<{ x: number; y: number; show: boolean } | null>(
    null,
  );

  let sortColumn = $state<string | null>(null);
  let sortDirection = $state<"asc" | "desc" | null>(null);
  let whereClause = $state<string>("");
  let whereError = $state<string | null>(null);

  async function handleWhereEnter() {
    offset = 0;
    whereError = null;
    if (fetchData) {
      try {
        await fetchData(offset, limit, sortColumn, sortDirection, whereClause);
      } catch (e: any) {
        whereError = typeof e === "string" ? e : e.message || String(e);
      }
    }
  }

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
    whereClause = "";
    whereError = null;
    historyManager.clear();
  });

  function commitEdit(
    newVal: string,
    direction: "none" | "next" | "down" = "none",
  ) {
    if (editingCell) {
      if (activeTableData?.rows && activeTableData.columns) {
        let updatedRows = [...activeTableData.rows];
        let modified = false;
        const ops: HistoryOp[] = [];

        // Apply to all selected elements
        // Only apply if the current cell is actually in the selection! (If there's no selection, just apply to the cell itself)
        const isEditingCellSelected =
          selectedRows[editingCell.r] ||
          selectedCols[editingCell.c] ||
          selectedCells[`${editingCell.r}-${editingCell.c}`];

        if (isEditingCellSelected) {
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

                  let origDict: Record<string, string> = {};
                  activeTableData.columns.forEach(
                    (col: string[], idx: number) => {
                      // Reconstruct original from activeTableData before it is clobbered
                      origDict[col[1]] = activeTableData.rows[r][idx];
                    },
                  );

                  // Decouple the row array to avoid mutating original state directly
                  if (updatedRows[r] === activeTableData.rows[r]) {
                    updatedRows[r] = [...activeTableData.rows[r]];
                  }

                  updatedRows[r][c - 1] = newVal;
                  modified = true;

                  // Also record in TransactionManager
                  let rowDict: Record<string, string> = {};
                  activeTableData.columns.forEach(
                    (col: string[], idx: number) => {
                      rowDict[col[1]] = updatedRows[r][idx]; // Record with updated values
                    },
                  );

                  let pkDict: Record<string, string> | null = null;
                  if (
                    activeTableData.primaryKeys &&
                    activeTableData.primaryKeys.length > 0
                  ) {
                    pkDict = {};
                    activeTableData.primaryKeys.forEach((pk: string) => {
                      let colIdx = activeTableData.columns.findIndex(
                        (c: string[]) => c[1] === pk,
                      );
                      if (colIdx >= 0) {
                        pkDict![pk] = activeTableData.rows[r][colIdx];
                      }
                    });
                  }

                  transactionManager.updateRow(r, pkDict, origDict, rowDict);
                }
              }
            }
          }
        } else {
          // Just apply to the single cell since the selection doesn't cover it
          const r = editingCell.r;
          const c = editingCell.c;
          const oldVal = updatedRows[r][c - 1];
          if (oldVal !== newVal) {
            ops.push({ r, c, oldVal, newVal });

            let origDict: Record<string, string> = {};
            activeTableData.columns.forEach((col: string[], idx: number) => {
              origDict[col[1]] = activeTableData.rows[r][idx];
            });

            if (updatedRows[r] === activeTableData.rows[r]) {
              updatedRows[r] = [...activeTableData.rows[r]];
            }

            updatedRows[r][c - 1] = newVal;
            modified = true;

            let rowDict: Record<string, string> = {};
            activeTableData.columns.forEach((col: string[], idx: number) => {
              rowDict[col[1]] = updatedRows[r][idx];
            });

            let pkDict: Record<string, string> | null = null;
            if (
              activeTableData.primaryKeys &&
              activeTableData.primaryKeys.length > 0
            ) {
              pkDict = {};
              activeTableData.primaryKeys.forEach((pk: string) => {
                let colIdx = activeTableData.columns.findIndex(
                  (c: string[]) => c[1] === pk,
                );
                if (colIdx >= 0) {
                  pkDict![pk] = activeTableData.rows[r][colIdx];
                }
              });
            }
            transactionManager.updateRow(r, pkDict, origDict, rowDict);
          }
        }

        if (modified) {
          historyManager.push(ops);
          activeTableData.rows = updatedRows;
          transactionChangesMap = new Map(transactionManager["changes"]); // Force reactivity
        }
      }

      if (activeTableData?.columns && activeTableData?.rows) {
        if (direction === "next") {
          let newC = editingCell.c + 1;
          let newR = editingCell.r;

          // wrap around to next row if we hit the end of the columns
          if (newC > activeTableData.columns.length) {
            newC = 1;
            newR += 1;
          }

          if (newR < activeTableData.rows.length) {
            editingCell = { r: newR, c: newC };
          } else {
            editingCell = null;
          }
        } else if (direction === "down") {
          let newR = editingCell.r + 1;

          if (newR < activeTableData.rows.length) {
            editingCell = { r: newR, c: editingCell.c };
          } else {
            editingCell = null;
          }
        } else {
          blurEditingInput();
          editingCell = null;
        }
      } else {
        blurEditingInput();
        editingCell = null;
      }
    }
  }

  function cancelEdit() {
    blurEditingInput();
    editingCell = null;
  }

  /**
   * Blur the active cell editor input and redirect focus to the scroll container
   * so the browser doesn't try to restore focus (and scroll) to the deleted input element.
   */
  function blurEditingInput() {
    const input =
      document.querySelector<HTMLInputElement>(".cell-editor-input");
    if (input) {
      // Move focus to the scroll container (which is non-scrolling) before removing the input
      scrollContainerEl?.focus({ preventScroll: true });
    }
  }

  async function handleKeyDown(e: KeyboardEvent) {
    // Escape mapping
    if (isShortcut(e, Shortcuts.Escape)) {
      if (editingCell) {
        blurEditingInput();
        editingCell = null;
        return;
      }

      const hasSelection =
        Object.keys(selectedCells).length > 0 ||
        Object.keys(selectedRows).length > 0 ||
        Object.keys(selectedCols).length > 0 ||
        selectionAnchor !== null;

      if (hasSelection) {
        baseSelectedRows = {};
        baseSelectedCols = {};
        baseSelectedCells = {};
        selectionAnchor = null;
        return;
      }

      // No edit and no selection, ask to discard changes
      if (transactionManager.hasChanges()) {
        const doDiscard = await confirm(
          "You have uncommitted changes. Are you sure you want to discard them?",
          { title: "Discard Changes", kind: "warning" },
        );
        if (doDiscard) {
          transactionManager.clear();
          transactionChangesMap = new Map();
          if (fetchData)
            fetchData(offset, limit, sortColumn, sortDirection, whereClause);
        }
      }
      return;
    }

    if (isShortcut(e, Shortcuts.Refresh)) {
      e.preventDefault();
      await handleRefresh();
      return;
    }

    // Undo/Redo intercept
    if (
      (e.ctrlKey || e.metaKey) &&
      (e.key.toLowerCase() === "z" || e.key.toLowerCase() === "y")
    ) {
      // Don't intercept if editing
      if (editingCell) return;
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA" ||
        (document.activeElement as HTMLElement)?.isContentEditable
      )
        return;

      e.preventDefault();

      if (e.key.toLowerCase() === "z" && !e.shiftKey) {
        if (activeTableData && historyManager.undo(activeTableData.rows)) {
          activeTableData.rows = [...activeTableData.rows];
        }
      } else {
        // Redo (Ctrl+Y or Ctrl+Shift+Z)
        if (activeTableData && historyManager.redo(activeTableData.rows)) {
          activeTableData.rows = [...activeTableData.rows];
        }
      }
      return;
    }
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "a") {
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA" ||
        (document.activeElement as HTMLElement)?.isContentEditable
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

    if (isShortcut(e, Shortcuts.Delete) || isShortcut(e, Shortcuts.Backspace)) {
      if (
        document.activeElement?.tagName === "INPUT" ||
        document.activeElement?.tagName === "TEXTAREA" ||
        (document.activeElement as HTMLElement)?.isContentEditable
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
      document.activeElement?.tagName === "TEXTAREA" ||
      (document.activeElement as HTMLElement)?.isContentEditable
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
      document.activeElement?.tagName === "TEXTAREA" ||
      (document.activeElement as HTMLElement)?.isContentEditable
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
    // If user clicks on an active input (like CellEditor or Toolbar), don't trigger cell selection drag
    const target = e.target as HTMLElement;
    if (
      target &&
      (target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.tagName === "SELECT" ||
        target.isContentEditable)
    ) {
      return;
    }

    if (e.button !== 0) return; // only left click

    if (editingCell) {
      commitEdit(
        document.querySelector(".cell-editor-input")
          ? (document.querySelector(".cell-editor-input") as HTMLInputElement)
              .value
          : "",
      );
      // We do not return here! We want the click to count as starting a new selection
      // on the cell we just clicked!
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

      const wasMultiDrag =
        Object.keys(dragSelectedCells).length > 0 ||
        Object.keys(dragSelectedRows).length > 0 ||
        Object.keys(dragSelectedCols).length > 0;

      baseSelectedRows = { ...baseSelectedRows, ...dragSelectedRows };
      baseSelectedCols = { ...baseSelectedCols, ...dragSelectedCols };
      baseSelectedCells = { ...baseSelectedCells, ...dragSelectedCells };
      dragSelectedRows = {};
      dragSelectedCols = {};
      dragSelectedCells = {};

      // Single click selection handled by handleMouseDown; we intentionally do not set `editingCell` here.
      // Doing so would enter edit mode immediately on a single click, which prevents copying.
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
    // Only apply drag selections if we actually moved from the anchor
    if (!(anchor.r === current.r && anchor.c === current.c)) {
      dragSelectedRows = newDragRows;
      dragSelectedCols = newDragCols;
      dragSelectedCells = newDragCells;
    } else {
      dragSelectedRows = {};
      dragSelectedCols = {};
      dragSelectedCells = {};
    }
  }

  function getShorthandType(rawType: string) {
    if (!rawType) return "";
    const lower = rawType.toLowerCase();
    if (lower.startsWith("timestamp with time zone")) return "timestamptz";
    if (lower.startsWith("timestamp without time zone")) return "timestamp";
    if (lower.startsWith("character varying")) return "varchar";
    if (lower.startsWith("double precision")) return "float8";
    if (lower.startsWith("real")) return "float4";
    if (lower.startsWith("boolean")) return "bool";
    if (lower.startsWith("integer")) return "int";
    if (lower.startsWith("smallint")) return "int2";
    if (lower.startsWith("bigint")) return "int8";
    return rawType;
  }

  let columns = $derived.by(() => {
    let cols: ColumnDef<any>[] = [
      {
        id: "index",
        header: "#",
        accessorFn: (_row, index) =>
          index + 1 + ((activeTableData?.currentPage || 1) - 1) * 100,
        size: 50,
        minSize: 30,
      },
    ];

    if (activeTableData?.columns) {
      activeTableData.columns.forEach((col: any) => {
        let colName = col[1];
        let rawType = col[2];
        let isPk = activeTableData.primaryKeys?.includes(colName) ?? false;
        let isFk = activeTableData.foreignKeys?.includes(colName) ?? false;

        cols.push({
          accessorKey: colName,
          header: colName,
          meta: { rawType: getShorthandType(rawType), isPk, isFk },
          cell: (info: CellContext<any, any>) => info.getValue(),
          size: 150,
          minSize: 50,
        });
      });
    }
    return cols;
  });

  function handleAddRow() {
    if (!activeTableData?.columns) return;

    // Create an array of empty strings matching column count
    const emptyRowArray = new Array(activeTableData.columns.length).fill("");
    const newIndex = activeTableData.rows.length;

    // Clear any previous selections that might cross over index references
    baseSelectedRows = {};
    baseSelectedCols = {};
    baseSelectedCells = {};
    selectionAnchor = null;
    editingCell = null;

    // Append to underlying data
    activeTableData.rows = [...activeTableData.rows, emptyRowArray];

    // Record in TransactionManager
    let newRowDict: Record<string, string> = {};
    activeTableData.columns.forEach((col: string[]) => {
      newRowDict[col[1]] = "";
    });

    transactionManager.addRow(newIndex, newRowDict);
    transactionChangesMap = new Map(transactionManager["changes"]);

    // Scroll table to the bottom slowly or instantly
    setTimeout(() => {
      const container = document.querySelector(".table-scroll-container");
      if (container) {
        container.scrollTop = container.scrollHeight;
      }
    }, 50);
  }

  function handleRemoveRow() {
    // Determine selected rows to delete
    let rowsToDelete = Object.keys(selectedRows).map(Number);

    // If no explicit row selection, check cell selection
    if (rowsToDelete.length === 0) {
      const cellKeys = Object.keys(selectedCells);
      if (cellKeys.length > 0) {
        // Extract unique row indices from selected cells
        const rowsSet = new Set(cellKeys.map((k) => Number(k.split("-")[0])));
        rowsToDelete = Array.from(rowsSet);
      }
    }

    if (rowsToDelete.length === 0 || !activeTableData?.columns) return;

    rowsToDelete.forEach((r) => {
      let origDict: Record<string, string> = {};
      activeTableData.columns.forEach((col: string[], idx: number) => {
        origDict[col[1]] = activeTableData.rows[r][idx];
      });

      let pkDict: Record<string, string> | null = null;
      if (
        activeTableData.primaryKeys &&
        activeTableData.primaryKeys.length > 0
      ) {
        pkDict = {};
        activeTableData.primaryKeys.forEach((pk: string) => {
          let colIdx = activeTableData.columns.findIndex(
            (c: string[]) => c[1] === pk,
          );
          if (colIdx >= 0) {
            pkDict![pk] = activeTableData.rows[r][colIdx];
          }
        });
      }

      transactionManager.deleteRow(r, pkDict, origDict);
    });

    transactionChangesMap = new Map(transactionManager["changes"]);
    baseSelectedRows = {};
    baseSelectedCells = {};
  }

  let showPreviewModal = $state(false);
  let previewQueries = $state<string[]>([]);

  async function handlePreview() {
    const changes = transactionManager.getAllChanges();
    if (changes.length === 0) {
      notificationMsg.set({
        type: NOTIFICATION_TYPE_SUCCESS,
        message: "No changes to preview.",
      });
      return;
    }

    try {
      let columnTypesDict: Record<string, string> = {};
      activeTableData.columns.forEach((col: any) => {
        columnTypesDict[col[1]] = col[2];
      });

      const payload = {
        database_name: activeTableData.dbName,
        schema_name: activeTableData.schemaName,
        table_name: activeTableData.tableName,
        changes: transactionManager.getAllChanges(),
        column_types: columnTypesDict,
      };

      const res: any = await invoke("generate_preview_queries_cmd", {
        reqPayload: payload,
      });

      if (res.error_code) {
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: res.frontend_msg || "Generate preview failed",
        });
      } else {
        previewQueries = res.data || [];
        showPreviewModal = true;
      }
    } catch (e) {
      console.error(e);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "An unexpected error occurred during preview generation.",
      });
    }
  }

  async function handleCommit() {
    const changes = transactionManager.getAllChanges();
    console.log("Committing changes:", changes);

    if (changes.length === 0) {
      notificationMsg.set({
        type: NOTIFICATION_TYPE_SUCCESS,
        message: "No changes to commit.",
      });
      return;
    }

    // Safeguard: Prevent empty rows
    for (const change of changes) {
      if (change.type === "INSERT" || change.type === "UPDATE") {
        let isAllEmpty = true;
        if (change.newValues) {
          for (const key of Object.keys(change.newValues)) {
            const val = change.newValues[key];
            if (val !== undefined && val !== null && val.trim() !== "") {
              isAllEmpty = false;
              break;
            }
          }
        }
        if (isAllEmpty) {
          notificationMsg.set({
            type: NOTIFICATION_TYPE_ERROR,
            message:
              "Cannot commit an empty row. Please fill in at least one cell or delete the empty row.",
          });
          return;
        }
      }
    }

    try {
      let columnTypesDict: Record<string, string> = {};
      activeTableData.columns.forEach((col: any) => {
        columnTypesDict[col[1]] = col[2];
      });

      const payload = {
        database_name: activeTableData.dbName,
        schema_name: activeTableData.schemaName,
        table_name: activeTableData.tableName,
        changes: transactionManager.getAllChanges(),
        column_types: columnTypesDict,
      };

      const res: any = await invoke(INVOKE_COMMIT_TRANSACTION, {
        reqPayload: payload,
      });

      if (res.error_code) {
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: res.frontend_msg || "Commit failed",
        });
      } else {
        notificationMsg.set({
          type: NOTIFICATION_TYPE_SUCCESS,
          message: "Transaction committed!",
        });
        transactionManager.clear();
        transactionChangesMap = new Map();
        if (fetchData)
          fetchData(offset, limit, sortColumn, sortDirection, whereClause); // Refresh UI
      }
    } catch (e) {
      console.error(e);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "An unexpected error occurred during commit.",
      });
    }
  }

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

  let columnSizing = $state<Record<string, number>>({});

  function autoFitColumn(columnId: string) {
    if (!activeTableData?.columns || !activeTableData?.rows) return;

    let colIndex = -1;
    if (columnId === "index") {
      const maxIndexStr = String(
        startIndex +
          visibleRows.length +
          1 +
          ((activeTableData?.currentPage || 1) - 1) * 100,
      );
      columnSizing = {
        ...columnSizing,
        [columnId]: Math.max(50, maxIndexStr.length * 10 + 24),
      };
      return;
    } else {
      colIndex = activeTableData.columns.findIndex(
        (c: string[]) => c[1] === columnId,
      );
    }

    if (colIndex === -1) return;

    let maxLength = columnId.length;
    for (const row of visibleRows) {
      const rowData = activeTableData.rows[row.index];
      if (rowData && rowData[colIndex]) {
        maxLength = Math.max(maxLength, String(rowData[colIndex]).length);
      }
    }

    const newWidth = Math.min(800, Math.max(50, maxLength * 8 + 48)); // +48 for padding and sort icon
    columnSizing = { ...columnSizing, [columnId]: newWidth };
  }

  let table = createTable({
    get data() {
      return data;
    },
    get columns() {
      return columns;
    },
    enableColumnResizing: true,
    columnResizeMode: "onChange",
    state: {
      get columnSizing() {
        return columnSizing;
      },
    },
    onColumnSizingChange: (updater) => {
      if (typeof updater === "function") {
        columnSizing = updater(columnSizing);
      } else {
        columnSizing = updater;
      }
    },
    getCoreRowModel: getCoreRowModel(),
    renderFallbackValue: null,
  });

  // Virtualization state
  let scrollTop = $state(0);
  let containerHeight = $state(800);
  let scrollContainerEl = $state<HTMLElement | null>(null);
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
  {#if isRefreshing}
    <div class="loader-overlay">
      <Loader loaderActive={true} width="32px" height="32px" color="#3b82f6" />
    </div>
  {/if}

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
      columns={activeTableData?.columns?.map((c: string[]) => c[1]) || []}
      bind:whereClause
      {whereError}
      onWhereEnter={handleWhereEnter}
      {gotoLast}
      {isRefreshing}
      onRefresh={handleRefresh}
      onAddRow={handleAddRow}
      onRemoveRow={handleRemoveRow}
      onCommit={handleCommit}
      onPreview={handlePreview}
      onRevert={handleRevert}
      hasChanges={transactionChangesMap.size > 0}
      {hasSelection}
      onExport={handleExport}
    />

    <ExportModal
      bind:showModal={showExportModal}
      columns={activeTableData.columns.map((c: string[]) => c[1])}
      data={activeTableData.rows.map((row: string[]) => {
        let rowObj: Record<string, any> = {};
        activeTableData.columns.forEach((col: string[], idx: number) => {
          rowObj[col[1]] = row[idx];
        });
        return rowObj;
      })}
      onClose={() => (showExportModal = false)}
    />

    <div
      class="table-scroll-container"
      bind:this={scrollContainerEl}
      onscroll={(e) => {
        if (editingCell) {
          const input = document.querySelector<
            HTMLInputElement | HTMLTextAreaElement
          >(".cell-editor-input");
          if (input) {
            commitEdit(input.value, "none");
          } else {
            editingCell = null;
          }
        }
        scrollTop = e.currentTarget.scrollTop;
      }}
      bind:clientHeight={containerHeight}
      tabindex="-1"
    >
      <table style="table-layout: fixed; width: {table.getTotalSize()}px;">
        <colgroup>
          {#each table.getFlatHeaders() as header (header.id)}
            <col style="width: {header.getSize()}px;" />
          {/each}
        </colgroup>
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
                      <span class="header-text">
                        <span
                          class:pk-col={(header.column.columnDef.meta as any)
                            ?.isPk}
                        >
                          {header.column.columnDef.header}
                        </span>
                        {#if (header.column.columnDef.meta as any)?.isPk || (header.column.columnDef.meta as any)?.isFk}
                          <span class="datatype-text">
                            ({#if (header.column.columnDef.meta as any)?.isPk && (header.column.columnDef.meta as any)?.isFk}pk,
                              fk
                            {:else if (header.column.columnDef.meta as any)?.isPk}pk
                            {:else}fk{/if})
                          </span>
                        {/if}
                        {#if (header.column.columnDef.meta as any)?.rawType}
                          <span class="datatype-text"
                            >({(header.column.columnDef.meta as any)
                              .rawType})</span
                          >
                        {/if}
                      </span>
                      {#if header.column.id !== "index"}
                        <button
                          class="sort-btn"
                          onmousedown={(e) => {
                            e.stopPropagation();
                          }}
                          onclick={async (e) => {
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
                            if (fetchData)
                              await fetchData(
                                offset,
                                limit,
                                sortColumn,
                                sortDirection,
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
                    <!-- svelte-ignore a11y_no_static_element_interactions -->
                    <div
                      class="resizer"
                      class:is-resizing={header.column.getIsResizing()}
                      onmousedown={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        const handler = header.getResizeHandler();
                        handler?.(e);
                      }}
                      ondblclick={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        autoFitColumn(header.column.id);
                      }}
                    ></div>
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
            {@const change = transactionChangesMap.get(r)}
            <tr
              class:selected-row={selectedRows[r]}
              class:inserted-row={change?.type === "INSERT"}
              class:updated-row={change?.type === "UPDATE"}
              class:deleted-row={change?.type === "DELETE"}
            >
              {#each row.getVisibleCells() as cell, c (cell.id)}
                {@const colName = cell.column.id}
                {@const isModified = change
                  ? change.type === "UPDATE"
                    ? change.newValues?.[colName] !==
                      change.originalRow?.[colName]
                    : change.type === "INSERT"
                      ? change.newValues?.[colName] !== ""
                      : false
                  : false}
                <td
                  class:selected-cell={selectedCells[`${r}-${c}`] ||
                    selectedCols[c]}
                  class:modified-cell={colName !== "index" && isModified}
                  class:editing-cell={editingCell?.r === r &&
                    editingCell?.c === c}
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
                  <div class="cell-content-wrapper">
                    <span
                      style="visibility: {editingCell?.r === r &&
                      editingCell?.c === c
                        ? 'hidden'
                        : 'visible'}"
                    >
                      {cell.getValue()}
                    </span>
                    {#if colName === "index" && change?.type === "DELETE"}
                      <button
                        class="row-revert-btn"
                        title="Revert Deletion"
                        aria-label="Revert Deletion"
                        onclick={(e) => {
                          e.stopPropagation();
                          transactionManager.revertChange(r);
                          transactionChangesMap = new Map(
                            transactionManager["changes"],
                          );
                        }}
                      >
                        <i class="fa-solid fa-rotate-left"></i>
                      </button>
                    {/if}
                  </div>
                  {#if editingCell?.r === r && editingCell?.c === c}
                    {@const colDef = activeTableData.columns.find(
                      (cDef: any) => cDef[1] === colName,
                    )}
                    {@const jsType = colDef ? colDef[0] : "string"}
                    {@const rawType = colDef ? colDef[2] : "text"}
                    <CellEditor
                      initialValue={cell.getValue() as string}
                      onCommit={commitEdit}
                      onCancel={cancelEdit}
                      {jsType}
                      {rawType}
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
          if (fetchData)
            fetchData(offset, limit, sortColumn, sortDirection, whereClause);
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

  <QueryPreviewModal
    show={showPreviewModal}
    queries={previewQueries}
    onClose={() => {
      showPreviewModal = false;
    }}
    onSubmit={() => {
      showPreviewModal = false;
      handleCommit();
    }}
  />
</div>

<style>
  .datatable-main-container {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: #ffffff;
    overflow: hidden;
    position: relative;
  }

  .loader-overlay {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 100;
    background: rgba(255, 255, 255, 0.7);
    padding: 16px;
    border-radius: 8px;
    display: flex;
    justify-content: center;
    align-items: center;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .table-scroll-container {
    flex: 1 1 0;
    min-height: 0;
    min-width: 0;
    overflow: auto;
    width: 100%;
    overscroll-behavior: contain;
    overflow-anchor: none;
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
    font-family: "JetBrains Mono", monospace;
    height: 32px;
    box-sizing: border-box;
  }

  tbody tr {
    height: 32px;
    box-sizing: border-box;
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

  /* Uncommitted Changes Highlight Row Gradients */
  tbody tr.inserted-row td,
  tbody tr.updated-row td {
    background: linear-gradient(
      to bottom,
      #fff1f2,
      #ffe4e6
    ) !important; /* rose-50 to rose-100 */
    box-shadow:
      inset 0 2px 0 0 #fb7185,
      /* rose-400 */ inset 0 -2px 0 0 #e11d48; /* rose-600 */
  }

  tbody tr.inserted-row td:first-child,
  tbody tr.updated-row td:first-child {
    background: linear-gradient(
      to bottom,
      #ffe4e6,
      #fecdd3
    ) !important; /* rose-100 to rose-200 */
    box-shadow:
      inset 0 2px 0 0 #fb7185,
      inset 0 -2px 0 0 #e11d48,
      inset 2px 0 0 0 #fb7185;
  }

  tbody tr.inserted-row td:last-child,
  tbody tr.updated-row td:last-child {
    box-shadow:
      inset 0 2px 0 0 #fb7185,
      inset 0 -2px 0 0 #e11d48,
      inset -2px 0 0 0 #e11d48;
  }

  /* Deleted State Styling */
  tbody tr.deleted-row td {
    background-color: #fef2f2 !important; /* very light red */
    color: #9ca3af !important; /* muted grey text */
    text-decoration: line-through;
    box-shadow: none !important;
    pointer-events: none; /* Make deleted rows non-interactive */
  }

  tbody tr.deleted-row td:first-child {
    pointer-events: auto; /* allow clicking revert in index col */
    background-color: #fee2e2 !important;
  }

  tbody tr.deleted-row td.modified-cell {
    background-color: #fef2f2 !important;
    color: #9ca3af !important;
    outline: none !important;
  }

  /* Uncommitted Changes Highlight - Modified Cell (Highest Priority) */
  tbody tr td.modified-cell {
    background-color: #fca5a5 !important; /* red-300 */
    color: #991b1b !important; /* red-800 - dark red text */
    outline: 2px solid #ef4444 !important; /* red-500 */
    outline-offset: -2px;
    box-shadow: none !important; /* ensure row gradients don't overlap the cell */
    z-index: 2; /* Ensure it stays above row gradients */
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

  .pk-col {
    color: #b45309; /* Yellow/amber text for primary keys */
  }

  .datatype-text {
    color: #4b5563; /* grey-600 */
    font-size: 11px;
    font-weight: 500;
    margin-left: 4px;
  }

  .cell-content-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
    height: 100%;
    gap: 4px;
  }

  .row-revert-btn {
    background: transparent;
    border: none;
    color: #ef4444; /* red-500 */
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition:
      background-color 0.2s,
      color 0.2s;
  }

  .row-revert-btn:hover {
    background-color: #fee2e2; /* red-100 */
    color: #b91c1c; /* red-700 */
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

  td.editing-cell {
    overflow: visible !important;
    z-index: 50 !important;
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

  .resizer {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    width: 5px;
    background-color: transparent;
    cursor: col-resize;
    user-select: none;
    z-index: 10;
  }

  .resizer:hover,
  .resizer.is-resizing {
    background-color: #3b82f6;
  }
</style>
