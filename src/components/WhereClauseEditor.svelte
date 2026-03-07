<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    EditorView,
    keymap,
    placeholder as cmPlaceholder,
  } from "@codemirror/view";
  import { EditorState, Compartment } from "@codemirror/state";
  import { linter, lintGutter, type Diagnostic } from "@codemirror/lint";
  import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
  import {
    autocompletion,
    completionKeymap,
    closeBrackets,
    closeBracketsKeymap,
    type CompletionContext,
    type CompletionResult,
  } from "@codemirror/autocomplete";
  import { sql, StandardSQL } from "@codemirror/lang-sql";

  let {
    value = $bindable(""),
    columns = [],
    errorMessage = null,
    onEnter = () => {},
  } = $props<{
    value?: string;
    columns?: string[];
    errorMessage?: string | null;
    onEnter?: () => void;
  }>();

  let editorContainer: HTMLDivElement;
  let view: EditorView | undefined;

  // Custom theme for JetBrains Mono and orange cursor
  const customTheme = EditorView.theme({
    "&": {
      color: "#111827",
      backgroundColor: "transparent",
      fontFamily: "'JetBrains Mono', monospace",
      fontSize: "13px",
      minHeight: "24px",
    },
    ".cm-content": {
      padding: "2px 0",
      caretColor: "#ea580c", // Orange cursor
    },
    "&.cm-focused .cm-cursor": {
      borderLeftColor: "#ea580c",
    },
    "&.cm-focused": {
      outline: "none",
    },
    ".cm-placeholder": {
      color: "#9ca3af",
    },
    ".cm-scroller": {
      overflowX: "auto",
      overflowY: "hidden",
      fontFamily: "'JetBrains Mono', monospace",
    },
    ".cm-tooltip": {
      fontFamily: "'JetBrains Mono', monospace",
      fontSize: "12px",
    },
  });

  function getCompletions(context: CompletionContext): CompletionResult | null {
    let word = context.matchBefore(/\w*/);
    if (!word || (word.from === word.to && !context.explicit)) return null;
    return {
      from: word.from,
      options: columns.map((col: string) => ({
        label: col,
        type: "property",
        info: "Column",
      })),
      validFor: /^\w*$/,
    };
  }

  // Keymap to prevent multiline and trigger search on enter
  const extensionKeymap = keymap.of([
    {
      key: "Enter",
      run: () => {
        onEnter();
        return true;
      },
    },
    ...defaultKeymap,
    ...historyKeymap,
    ...completionKeymap,
    ...closeBracketsKeymap,
  ]);

  const updateListener = EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      const newVal = update.state.doc.toString();
      // Handle pasted newlines by stripping them
      if (newVal.includes("\n")) {
        const cleaned = newVal.replace(/\n/g, " ");
        setTimeout(() => {
          if (view) {
            view.dispatch({
              changes: {
                from: 0,
                to: view.state.doc.length,
                insert: cleaned,
              },
            });
            value = cleaned;
          }
        }, 0);
      } else {
        value = newVal;
      }
    }
  });

  const linterCompartment = new Compartment();

  function getLinterExtension(errorMsg: string | null) {
    if (!errorMsg) {
      return linter(() => []);
    }
    return linter((view) => {
      let diagnostics: Diagnostic[] = [];
      if (view.state.doc.length > 0) {
        diagnostics.push({
          from: 0,
          to: view.state.doc.length,
          severity: "error",
          message: errorMsg,
        });
      }
      return diagnostics;
    });
  }

  onMount(() => {
    let state = EditorState.create({
      doc: value,
      extensions: [
        history(),
        closeBrackets(),
        lintGutter(),
        linterCompartment.of(getLinterExtension(errorMessage)),
        autocompletion({ override: [getCompletions] }),
        sql({ dialect: StandardSQL }),
        customTheme,
        extensionKeymap,
        cmPlaceholder("condition..."),
        updateListener,
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });
  });

  // Track value changes from outside (e.g. clearing it)
  $effect(() => {
    if (view && value !== view.state.doc.toString()) {
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: value },
      });
    }
  });

  $effect(() => {
    if (view) {
      view.dispatch({
        effects: linterCompartment.reconfigure(
          getLinterExtension(errorMessage),
        ),
      });
    }
  });

  onDestroy(() => {
    if (view) {
      view.destroy();
    }
  });
</script>

<div class="where-editor-wrapper">
  <div bind:this={editorContainer} class="cm-container"></div>
</div>

<style>
  .where-editor-wrapper {
    display: flex;
    align-items: center;
    width: 100%;
    min-width: 0;
    background: transparent;
    border-left: 2px solid #fca5a5;
    border-right: 1px solid #eee;
    transition: all 0.2s ease;
    padding-left: 8px;
    cursor: text;
  }

  .where-editor-wrapper:focus-within {
    border-left: 2px solid #ea580c; /* darker orange when active */
    background: rgba(255, 255, 255, 0.4);
    border-radius: 0 4px 4px 0;
  }

  .cm-container {
    width: 100%;
    min-width: 0;
  }

  :global(.cm-editor.cm-focused) {
    outline: none !important;
  }
</style>
