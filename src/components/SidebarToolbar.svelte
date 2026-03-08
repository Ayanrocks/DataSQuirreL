<script lang="ts">
  import AddCircle from "../assets/icons/addCircle.svg?raw";
  import Refresh from "../assets/icons/refresh.svg?raw";
  import Disconnect from "../assets/icons/disconnect.svg?raw";
  import Console from "../assets/icons/terminal.svg?raw";
</script>

<div class="sidebar-toolbar">
  <div class="sidebar-toolbar-content w-full">
    <span class="icon add-circle scale-75" title="Add New Connection">
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html AddCircle}
    </span>
    <span class="icon refresh scale-75" title="Refresh">
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html Refresh}
    </span>
    <span class="icon disconnect scale-75" title="Disconnect">
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html Disconnect}
    </span>
    <span class="icon console scale-75" title="Open Query Console">
      <!-- eslint-disable-next-line svelte/no-at-html-tags -->
      {@html Console}
    </span>
  </div>
</div>

<style>
  .sidebar-toolbar {
    position: relative;
    top: 0;
    left: 0;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 30px;
    width: 100%;
    background-color: var(--accentColor);
  }

  .sidebar-toolbar-content {
    width: 100%;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: start;
    height: 100%;
  }

  .icon {
    color: #1d1b20;
    margin: 0 5px;
    cursor: pointer;
    fill: #1d1b20;
    border-radius: 5px;
  }
</style>
