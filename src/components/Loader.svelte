<script lang="ts">
  export let loaderActive: boolean = false;
  export let height: string = '20px';
  export let width: string = '20px';
  export let color: string = '#000';
</script>

{#if loaderActive === true}
  <span class="loading loading-spinner loading-xs" style="background-color: {color}; height:{height}; width:{width};"
  ></span>
{/if}

<style>
</style>
