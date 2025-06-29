<script lang="ts">
  import ExpandableArrowIcon from "../assets/icons/expandableArrow.svg?raw";
  import SchemaIcon from "../assets/icons/schema.svg?raw";
  import TableIcon from "../assets/icons/table.svg?raw";
  import PostgreSQLIcon from "../assets/icons/postgresql.svg?raw";
  import ArrowRightIcon from "../assets/icons/arrowRight.svg?raw";
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  let {
    entityName,
    isExpanded,
    entityType,
    hasChildren,
  }: { entityName: string; isExpanded: boolean; entityType: string; hasChildren: boolean } = $props();
</script>

<div class="sidebar-item">
  <div class="sidebar-item-content">
    {#if hasChildren}
      <button
        type="button"
        class="expandable-icon scale-55"
        onclick={() => {
          dispatch('toggle');
        }}
        aria-expanded={isExpanded}
      >
        {@html isExpanded ? ExpandableArrowIcon : ArrowRightIcon}
      </button>
    {:else}
      <div class="expandable-icon scale-55"></div>
    {/if}
    <div class="entity-icon">
      {#if entityType === "Schema"}
        <span class="icon-container scale-85">
          {@html SchemaIcon}
        </span>
      {:else if entityType === "Table"}
        <span class="icon-container scale-75 text-white">
          {@html TableIcon}
        </span>
      {:else if entityType === "postgresql"}
        <span class="icon-container scale-85">
          {@html PostgreSQLIcon}
        </span>
        <!-- {:else if entityName === "View"}
        {@html ViewIcon}
      {:else if entityName === "Function"}
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

  .icon-container {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    /* color: #1d1b20; */
    margin-left: 5px;
  }
</style>
