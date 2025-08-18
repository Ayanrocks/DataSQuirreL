<script lang="ts">
    import { activeTable } from "../stores";
  import SideBarItem from "./SideBarItem.svelte";
  import { slide } from "svelte/transition";

  export let item: {
    entityName: string;
    isExpanded: boolean;
    entityType: string;
    level: number;
    children: any[];
  };

  // Add parent context to track hierarchy
  export let parentContext: {
    databaseName?: string;
    schemaName?: string;
  } = {};

  function handleToggle() {
    item.isExpanded = !item.isExpanded;
  }

  function handleTableClick(entityType: string, fullPath: string) {
    if (entityType === "Table") {
      let dbComponents = fullPath.split("::")
      console.log("clicked", fullPath, dbComponents);

      // write logic to invoke fetch table data and render in the mainscreen 
      // set active table to the tablename 
      activeTable.set()
    }
  }

  // Build the current context based on the item type
  $: currentContext = {
    ...parentContext,
    ...(item.entityType === "postgresql" && { databaseName: item.entityName }),
    ...(item.entityType === "Schema" && { schemaName: item.entityName }),
  };

  // Get the full path for tables
  $: fullPath =
    item.entityType === "Table"
      ? `${currentContext.databaseName || "NULL"}::${currentContext.schemaName || "NULL"}::${item.entityName}`
      : "NULL";

</script>

<div class="sidebar-item" style="padding-left: {item.level * 15}px">
  <SideBarItem
    entityName={item.entityName}
    isExpanded={item.isExpanded}
    entityType={item.entityType}
    hasChildren={item.children && item.children.length > 0}
    toggle={handleToggle}
    {fullPath}
    {handleTableClick}
  />
  {#if item.children && item.children.length > 0 && item.isExpanded}
    <div transition:slide>
      {#each item.children as child}
        <svelte:self item={child} parentContext={currentContext} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .sidebar-item {
    margin: 2px 0;
  }
</style>
