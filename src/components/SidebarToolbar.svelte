<script lang="ts">
  import AddCircle from "../assets/icons/addCircle.svg?raw";
  import Refresh from "../assets/icons/refresh.svg?raw";
  import Disconnect from "../assets/icons/disconnect.svg?raw";
  import Console from "../assets/icons/terminal.svg?raw";
</script>

<div class="sidebar-toolbar">
  <div class="sidebar-toolbar-content w-full">
    <span class="icon add-circle">
      {@html AddCircle}
    </span>
    <span class="icon refresh">
      {@html Refresh}
    </span>
    <span class="icon disconnect">
      {@html Disconnect}
    </span>
    <span class="icon console">
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
    height: 40px;
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
    margin: 0 10px;
    cursor: pointer;
    fill: #1d1b20;
    border-radius: 5px;
    height: 18px;
    width: 18px;
  }
</style>
