<script lang="ts">
  export let loaderActive: boolean = false;
  export let height: string = "20px";
  export let width: string = "20px";
  export let color: string = "#000";
</script>

{#if loaderActive === true}
  <div class="loader-container mx-4">
    <div
      class="lds-dual-ring"
      style="height:{height};--width:{width}; --color:{color}"
    ></div>
  </div>
{/if}

<style>
  .lds-dual-ring {
    display: inline-block;
    width: var(--width);
    height: var(--height);
    border-radius: 50%;
    border: 2px solid var(--color);
    border-color: var(--color) transparent var(--color) transparent;
    animation: lds-dual-ring 1.2s cubic-bezier(1, 0, 0, 1) infinite;
  }

  @keyframes lds-dual-ring {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
