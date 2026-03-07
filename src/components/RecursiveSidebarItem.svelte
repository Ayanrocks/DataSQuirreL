<script lang="ts">
  import { activeTable } from "../stores";
  import SideBarItem from "./SideBarItem.svelte";
  import { slide } from "svelte/transition";
  import RecursiveSidebarItem from "./RecursiveSidebarItem.svelte";

  // export let item: {
  //   entityName: string;
  //   isExpanded: boolean;
  //   entityType: string;
  //   level: number;
  //   children: any[];
  // };

  let {
    item,
    parentContext,
    handleTableClick,
  }: {
    item: {
      entityName: string;
      isExpanded: boolean;
      entityType: string;
      level: number;
      children: any[];
    };
    parentContext: {
      databaseName?: string;
      schemaName?: string;
    };
    handleTableClick: (entityType: string, fullPath: string) => void;
  } = $props();

  function handleToggle() {
    item.isExpanded = !item.isExpanded;
  }

  // Build the current context based on the item type
  let currentContext = $derived({
    ...parentContext,
    ...(item.entityType === "postgresql" && { databaseName: item.entityName }),
    ...(item.entityType === "Schema" && { schemaName: item.entityName }),
  });

  // Get the full path for tables
  let fullPath = $derived(
    item.entityType === "Table"
      ? `${currentContext.databaseName || "NULL"}::${currentContext.schemaName || "NULL"}::${item.entityName}`
      : "NULL",
  );
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
      {#each item.children as child (child.entityName)}
        <RecursiveSidebarItem
          item={child}
          parentContext={currentContext}
          {handleTableClick}
        />
      {/each}
    </div>
  {/if}
</div>

<style>
  .sidebar-item {
    margin: 2px 0;
  }
</style>
