<script lang="ts">
  import SideBarItem from './SideBarItem.svelte';
  
  export let item: {
    entityName: string;
    isExpanded: boolean;
    entityType: string;
    level: number;
    children: any[];
  };
</script>

<div class="sidebar-item" style="padding-left: {item.level * 15}px">
  <SideBarItem
    entityName={item.entityName}
    isExpanded={item.isExpanded}
    entityType={item.entityType}
  />
  {#if item.children && item.children.length > 0}
    {#each item.children as child}
      <svelte:self item={child} />
    {/each}
  {/if}
</div>

<style>
  .sidebar-item {
    margin: 2px 0;
  }
</style> 