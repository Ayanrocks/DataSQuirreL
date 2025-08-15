<script lang="ts">
  import SideBarItem from "./SideBarItem.svelte";
  import { slide } from 'svelte/transition';

  export let item: {
    entityName: string;
    isExpanded: boolean;
    entityType: string;
    level: number;
    children: any[];
  };

  // Debug logging
  $: console.log('RecursiveSidebarItem item:', item);

  function handleToggle() {
    item.isExpanded = !item.isExpanded;
  }
</script>

<div class="sidebar-item" style="padding-left: {item.level * 15}px">
  <SideBarItem
    entityName={item.entityName}
    isExpanded={item.isExpanded}
    entityType={item.entityType}
    hasChildren={item.children && item.children.length > 0}
    on:toggle={handleToggle}
  />
  {#if item.children && item.children.length > 0 && item.isExpanded}
    <div transition:slide>
      {#each item.children as child}
        <svelte:self item={child} />
      {/each}
    </div>
  {/if}
</div>

<style>
  .sidebar-item {
    margin: 2px 0;
  }
</style>
