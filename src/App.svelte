<script>
  import { onMount, onDestroy } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  export const ssr = false;
  import InitScreen from "./routes/InitScreen.svelte";
  import MainScreen from "./routes/MainScreen.svelte";
  import Notification from "./components/Notification.svelte";
  import WindowControls from "./components/WindowControls.svelte";
  import AboutModal from "./components/AboutModal.svelte";
  import { currentPath } from "./lib/router";
  import "./App.css";

  let isAboutOpen = $state(false);
  let unlistenAbout;

  onMount(async () => {
    unlistenAbout = await listen("open-about-modal", () => {
      isAboutOpen = true;
    });
  });

  onDestroy(() => {
    if (unlistenAbout) unlistenAbout();
  });
</script>

<WindowControls />
<Notification />
<AboutModal isOpen={isAboutOpen} onClose={() => (isAboutOpen = false)} />
{#if $currentPath === "/dashboard"}
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
