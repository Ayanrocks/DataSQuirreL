<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    EditorView,
    keymap,
    placeholder as cmPlaceholder,
    lineNumbers,
    highlightActiveLineGutter,
    highlightActiveLine,
    dropCursor,
  } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { sql, StandardSQL } from "@codemirror/lang-sql";
  import {
    defaultKeymap,
    history,
    historyKeymap,
    indentWithTab,
  } from "@codemirror/commands";
  import {
    autocompletion,
    completionKeymap,
    closeBrackets,
    closeBracketsKeymap,
  } from "@codemirror/autocomplete";
  import { bracketMatching } from "@codemirror/language";

  import DataTable from "./DataTable.svelte";
  import ConsoleToolBar from "./ConsoleToolBar.svelte";

  let {
    content = $bindable(""),
    activeConsoleData = $bindable(null),
    isExecuting = false,
    executionTimeMs = null,
    hasUnsavedChanges = false,
    errorMessage = null,
    onExecute = () => {},
    onFormat = () => {},
    onSave = () => {},
  } = $props<{
    content: string | undefined;
    activeConsoleData: any;
    isExecuting?: boolean;
    executionTimeMs?: number | null;
    hasUnsavedChanges?: boolean;
    errorMessage?: string | null;
    onExecute?: (
      query: string,
      offset: number,
      limit: number,
      sortColumn: string | null,
      sortDirection: string | null,
      whereClause: string | null,
    ) => void;
    onFormat?: () => void;
    onSave?: () => void;
  }>();

  let editorContainer: HTMLDivElement;
  let view: EditorView | undefined;
  let hasSelection = $state(false);

  // Pagination and DataTable state
  let currentQuery = $state("");
  let currentOffset = $state(0);
  let currentLimit = $state(100);

  // Toast notification state
  let showToast = $state(false);
  let toastMessage = $state("");
  let toastTimer: any;

  // Resizing state
  let containerHeight = $state(100);
  let resizerActive = $state(false);
  let editorFlex = $state(1);
  let resultsFlex = $state(1);

  let parsedConsoleData = $derived.by(() => {
    if (!activeConsoleData) return null;

    // Support both snake_case (Rust serde default) and camelCase (Tauri plugin edge cases)
    const isSelect =
      activeConsoleData.is_select === true ||
      activeConsoleData.isSelect === true;
    const columns = Array.isArray(activeConsoleData.columns)
      ? activeConsoleData.columns
      : [];
    const rows = Array.isArray(activeConsoleData.rows)
      ? activeConsoleData.rows
      : [];
    const rowCount =
      activeConsoleData.row_count ?? activeConsoleData.rowCount ?? 0;
    const execTime =
      activeConsoleData.execution_time_ms ??
      activeConsoleData.executionTimeMs ??
      0;

    return { isSelect, columns, rows, rowCount, execTime };
  });

  let dataTableFormat = $derived.by(() => {
    if (!parsedConsoleData || !parsedConsoleData.isSelect) return null;
    return {
      id: "console_results",
      tableName: "Query Results",
      columns: parsedConsoleData.columns.map((c: string) => ["", c]),
      rows: parsedConsoleData.rows,
      rowCount: parsedConsoleData.rowCount,
      currentPage: Math.floor(currentOffset / currentLimit) + 1,
      maxPage: Math.ceil(parsedConsoleData.rowCount / currentLimit) || 1,
      currentLimit: currentLimit,
      currentOffset: currentOffset,
      primaryKeys: [],
      foreignKeys: [],
      schemaName: "",
      dbName: "",
    };
  });

  $effect(() => {
    if (!isExecuting && (activeConsoleData || errorMessage) && !showToast) {
      if (parsedConsoleData?.execTime) {
        toastMessage = `Query Executed Successfully in ${parsedConsoleData.execTime} ms`;
      } else if (errorMessage) {
        toastMessage = `Query Failed!`;
      } else {
        toastMessage = "Query Executed Successfully";
      }
      showToast = true;
      if (toastTimer) clearTimeout(toastTimer);
      toastTimer = setTimeout(() => {
        showToast = false;
      }, 3000);
    }
  });

  const customTheme = EditorView.theme({
    "&": {
      height: "100%",
      fontFamily: "'JetBrains Mono', monospace",
      fontSize: "13px",
    },
    ".cm-content": {
      padding: "10px 0",
      caretColor: "#ea580c",
    },
    "&.cm-focused .cm-cursor": {
      borderLeftColor: "#ea580c",
    },
    "&.cm-focused": {
      outline: "none",
    },
    ".cm-scroller": {
      overflow: "auto",
      fontFamily: "'JetBrains Mono', monospace",
    },
    ".cm-gutters": {
      backgroundColor: "#f9fafb",
      color: "#9ca3af",
      border: "none",
      borderRight: "1px solid #e5e7eb",
    },
    ".cm-activeLineGutter": {
      backgroundColor: "#e5e7eb",
      color: "#4b5563",
    },
    ".cm-activeLine": {
      backgroundColor: "rgba(229, 231, 235, 0.4)",
    },
    ".cm-selectionBackground": {
      backgroundColor: "rgba(59, 130, 246, 0.2) !important",
    },
  });

  const customKeymap = keymap.of([
    {
      key: "Mod-Enter",
      run: () => {
        handleRun();
        return true;
      },
    },
    {
      key: "Mod-Shift-Enter",
      run: () => {
        handleRunSelection();
        return true;
      },
    },
    {
      key: "Mod-s",
      run: () => {
        onSave();
        return true;
      },
      preventDefault: true,
    },
    indentWithTab,
    ...defaultKeymap,
    ...historyKeymap,
    ...completionKeymap,
    ...closeBracketsKeymap,
  ]);

  const updateListener = EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      content = update.state.doc.toString();
    }

    // Check if there is an active selection
    if (update.selectionSet || update.docChanged) {
      const selection = update.state.selection.main;
      hasSelection = !selection.empty;
    }
  });

  onMount(() => {
    let state = EditorState.create({
      doc: content,
      extensions: [
        lineNumbers(),
        highlightActiveLineGutter(),
        highlightActiveLine(),
        history(),
        dropCursor(),
        bracketMatching(),
        closeBrackets(),
        autocompletion(),
        sql({ dialect: StandardSQL }),
        customTheme,
        customKeymap,
        cmPlaceholder("-- Write your SQL query here..."),
        updateListener,
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });
  });

  $effect(() => {
    // Sync external changes to the editor (like formatting or initialization) if it doesn't match
    if (view && content !== view.state.doc.toString()) {
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: content },
      });
    }
  });

  onDestroy(() => {
    if (view) {
      view.destroy();
    }
  });

  function handleRun() {
    if (view) {
      currentQuery = view.state.doc.toString();
      currentOffset = 0;
      currentLimit = 100;
      onExecute(currentQuery, currentOffset, currentLimit, null, null, null);
    }
  }

  function handleRunSelection() {
    if (view && hasSelection) {
      const selection = view.state.selection.main;
      currentQuery = view.state.sliceDoc(selection.from, selection.to);
      currentOffset = 0;
      currentLimit = 100;
      onExecute(currentQuery, currentOffset, currentLimit, null, null, null);
    }
  }

  function fetchConsoleData(
    offset: number,
    limit: number | null,
    sortColumn?: string | null,
    sortDirection?: string | null,
    whereClause?: string | null,
  ) {
    currentOffset = offset;
    currentLimit = limit || 100;
    onExecute(
      currentQuery,
      currentOffset,
      currentLimit,
      sortColumn || null,
      sortDirection || null,
      whereClause || null,
    );
  }

  function handleResizeStart(e: MouseEvent) {
    resizerActive = true;
    document.body.style.cursor = "row-resize";
    document.body.style.userSelect = "none";
    window.addEventListener("mousemove", handleResizeMove);
    window.addEventListener("mouseup", handleResizeEnd);
  }

  function handleResizeMove(e: MouseEvent) {
    if (!resizerActive) return;
    const container = document.querySelector(".split-pane") as HTMLElement;
    if (container) {
      const rect = container.getBoundingClientRect();
      const topOffset = e.clientY - rect.top;
      const bottomOffset = rect.height - topOffset;
      if (topOffset > 50 && bottomOffset > 50) {
        editorFlex = topOffset;
        resultsFlex = bottomOffset;
      }
    }
  }

  function handleResizeEnd() {
    resizerActive = false;
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
    window.removeEventListener("mousemove", handleResizeMove);
    window.removeEventListener("mouseup", handleResizeEnd);
  }

  function handleFormat() {
    onFormat();
  }
</script>

<div class="console-container">
  <ConsoleToolBar
    {isExecuting}
    {executionTimeMs}
    {hasUnsavedChanges}
    {hasSelection}
    onRun={handleRun}
    onRunSelection={handleRunSelection}
    onFormat={handleFormat}
    {onSave}
  />

  <div class="split-pane">
    <div class="editor-pane" style="flex: {editorFlex};">
      <div bind:this={editorContainer} class="cm-container"></div>

      {#if showToast}
        <div
          class="toast-notification {errorMessage
            ? 'toast-error'
            : 'toast-success'}"
        >
          {toastMessage}
        </div>
      {/if}
    </div>

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="resizer" onmousedown={handleResizeStart}></div>

    <div class="results-pane" style="flex: {resultsFlex};">
      {#if isExecuting}
        <div class="status-message loading">
          <i class="fa-solid fa-circle-notch fa-spin"></i> Executing query...
        </div>
      {:else if errorMessage}
        <div class="status-message error">
          <i class="fa-solid fa-circle-exclamation"></i>
          <span class="error-text">{errorMessage}</span>
        </div>
      {:else if parsedConsoleData}
        {#if parsedConsoleData.isSelect}
          {#if parsedConsoleData.rows.length === 0}
            <div class="status-message success">
              Query executed successfully, but returned 0 rows.
            </div>
          {:else if dataTableFormat}
            <DataTable
              bind:activeTableData={dataTableFormat}
              fetchData={fetchConsoleData}
            />
          {/if}
        {:else}
          <div class="status-message success">
            <i class="fa-solid fa-check-circle"></i>
            Query executed successfully. Affected rows: {parsedConsoleData.rowCount}
          </div>
        {/if}
      {:else}
        <div class="status-message empty">
          Ready. Run a query to see results here.
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .console-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    width: 100%;
    background-color: #ffffff;
    overflow: hidden;
  }

  .split-pane {
    display: flex;
    flex-direction: column;
    height: calc(100% - 40px); /* minus toolbar */
    width: 100%;
  }

  .editor-pane {
    flex: 1; /* By default take half screen */
    min-height: 100px;
    position: relative;
    background-color: #ffffff;
    display: flex;
  }

  .cm-container {
    width: 100%;
    height: 100%;
  }

  .resizer {
    height: 6px;
    background-color: #e5e7eb;
    cursor: row-resize;
    z-index: 5;
    transition: background-color 0.15s;
    border-top: 1px solid #d1d5db;
    border-bottom: 1px solid #d1d5db;
  }

  .resizer:hover {
    background-color: #d1d5db;
  }

  .results-pane {
    flex: 1;
    min-height: 100px;
    background-color: #f9fafb;
    overflow: auto;
    position: relative;
    display: flex;
    flex-direction: column;
  }

  .status-message {
    padding: 20px;
    display: flex;
    align-items: flex-start;
    gap: 10px;
    font-size: 14px;
    font-family: "JetBrains Mono", monospace;
  }

  .empty {
    color: #6b7280;
    align-items: center;
    justify-content: center;
    height: 100%;
  }

  .loading {
    color: #4b5563;
  }

  .error {
    color: #b91c1c;
    background-color: #fef2f2;
    border-bottom: 1px solid #fecaca;
  }

  .success {
    color: #047857;
    background-color: #ecfdf5;
    border-bottom: 1px solid #d1fae5;
  }

  .error-text {
    white-space: pre-wrap;
    word-break: break-word;
  }

  .toast-notification {
    position: absolute;
    bottom: 20px;
    right: 20px;
    padding: 10px 15px;
    border-radius: 6px;
    color: white;
    font-size: 13px;
    font-family: "JetBrains Mono", monospace;
    z-index: 100;
    box-shadow:
      0 4px 6px -1px rgba(0, 0, 0, 0.1),
      0 2px 4px -1px rgba(0, 0, 0, 0.06);
    animation: fadein 0.3s forwards;
  }

  .toast-success {
    background-color: #059669; /* emerald-600 */
  }

  .toast-error {
    background-color: #dc2626; /* red-600 */
  }

  @keyframes fadein {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  :global(.console-container .cm-editor) {
    height: 100%;
    width: 100%;
  }
</style>
