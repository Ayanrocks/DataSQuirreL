<script>
  import InitConnection from "./views/InitConnection.svelte";
  // import MainScreen from "./views/MainScreen.svelte";
  import Notification from "./components/Notification.svelte";
  import "./tailwind.css"
</script>

<Notification />
<div id="router-view container">
  <InitConnection />
  <!-- <MainScreen /> -->
</div>

<style>
  #router-view {
    overflow: hidden;
  }
</style>
