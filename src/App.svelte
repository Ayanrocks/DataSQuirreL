<script>
  export const ssr = false;
  import InitScreen from "./routes/InitScreen.svelte";
  import MainScreen from "./routes/MainScreen.svelte";
  import Notification from "./components/Notification.svelte";
  import WindowControls from "./components/WindowControls.svelte";
  import { currentPath } from './lib/router';
  import "./App.css";
</script>

<WindowControls />
<Notification />
{#if $currentPath === '/dashboard'}
  <div id="router-view">
    <MainScreen />
  </div>
{:else}
  <div id="router-view">
    <InitScreen />
  </div>
{/if}

<style>
  #router-view {
    overflow: hidden;
    height: 100%;
    width: 100%;
  }
</style>
