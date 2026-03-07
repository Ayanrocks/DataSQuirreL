<script lang="ts">
  import ExpandableArrowIcon from "../assets/icons/expandableArrow.svg?raw";
  import SchemaIcon from "../assets/icons/schema.svg?raw";
  import TableIcon from "../assets/icons/table.svg?raw";
  import PostgreSQLIcon from "../assets/icons/postgresql.svg?raw";
  import ArrowRightIcon from "../assets/icons/arrowRight.svg?raw";
  import ConsoleIcon from "../assets/icons/console.svg?raw";
  import { isShortcut, Shortcuts } from "../lib/shortcuts";

  let {
    entityName,
    isExpanded,
    entityType,
    hasChildren,
    fullPath,
    toggle,
    handleTableClick,
  }: {
    entityName: string;
    isExpanded: boolean;
    entityType: string;
    hasChildren: boolean;
    fullPath: string;
    toggle: () => void;
    handleTableClick: (entityType: string, fullPath: string) => void;
  } = $props();

  const handleMouseTableClick = (e: MouseEvent) => {
    e.stopPropagation();
    if (entityType === "Table" || entityType === "Console") {
      e.preventDefault();
      handleTableClick(entityType, fullPath);
    } else {
      // Toggle expansion when simply clicking the row container of schemas
      if (hasChildren) {
        // Only toggle if directly clicking the container
        toggle();
      }
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (isShortcut(e, Shortcuts.Enter) || isShortcut(e, Shortcuts.Space)) {
      e.stopPropagation();
      e.preventDefault();
      if (entityType === "Table" || entityType === "Console") {
        handleTableClick(entityType, fullPath);
      } else if (hasChildren) {
        toggle();
      }
    }
  };
</script>

<div class="sidebar-item">
  <div
    class="sidebar-item-content"
    onclick={handleMouseTableClick}
    onkeydown={handleKeyDown}
    tabindex={entityType === "Table" || entityType === "Console" ? 0 : -1}
    aria-label={entityType === "Table"
      ? `Select table ${entityName}`
      : entityType === "Console"
        ? `Select console ${entityName}`
        : undefined}
    role={entityType === "Table" || entityType === "Console"
      ? "button"
      : hasChildren
        ? "treeitem"
        : undefined}
    aria-expanded={hasChildren ? isExpanded : undefined}
  >
    <div class="arrow-space scale-75">
      {#if hasChildren}
        <button
          type="button"
          class="expandable-icon"
          onclick={(e) => {
            e.stopPropagation();
            toggle();
          }}
          aria-expanded={isExpanded}
        >
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html isExpanded ? ExpandableArrowIcon : ArrowRightIcon}
        </button>
      {/if}
    </div>
    <div class="entity-icon">
      {#if entityType === "Schema"}
        <span class="icon-container">
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html SchemaIcon}
        </span>
      {:else if entityType === "Table"}
        <span class="icon-container text-white">
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html TableIcon}
        </span>
      {:else if entityType === "postgresql"}
        <span class="icon-container">
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html PostgreSQLIcon}
        </span>
      {:else if entityType === "Console"}
        <span class="icon-container">
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html ConsoleIcon}
        </span>
        <!-- {:else if entityName === "Function"}
        {@html FunctionIcon} -->
      {/if}
    </div>
    <div class="entity-name">
      {entityName}
    </div>
  </div>
</div>

<style>
  .sidebar-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: start;
    width: 100%;
    color: #1d1b20;
  }

  .entity-name {
    font-size: 14px;
    font-weight: 500;
    color: #1d1b20;
    margin-left: 5px;
  }

  .sidebar-item-content {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: start;
    width: 100%;
    min-height: 25px;
    padding-block: 2px;
    cursor: pointer;
  }

  .sidebar-item-content:hover {
    background-color: var(--accentColor);
  }

  .icon-container,
  .expandable-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    margin: 0 5px;
    /* color: #1d1b20; */
  }

  .arrow-space {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
