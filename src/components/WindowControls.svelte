<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { platform } from "@tauri-apps/plugin-os";
  import { onMount } from "svelte";

  let currentPlatform: string = "";
  const appWindow = getCurrentWindow();

  async function getPlatform(): Promise<string> {
    const platformName = await platform();
    console.log(platformName); // "windows", "macos", "linux", etc.
    return platformName;
  }

  onMount(async () => {
    try {
      currentPlatform = await getPlatform();
    } catch (e) {
      currentPlatform = ""; // fallback for SSR or error
    }
  });

  const minimize = () => {
    appWindow.minimize();
  };

  const maximizeOrUnmaximize = async () => {
    const maximized = await appWindow.isMaximized();
    if (maximized) {
      appWindow.unmaximize();
    } else {
      appWindow.maximize();
    }
    console.log(
      "Maximize/Unmaximize called. Window maximized state:",
      !maximized,
    );
  };

  const close = () => {
    appWindow.close();
  };
</script>

<!-- FIXME: Window is not draggable yet -->
<div
  class="window-controls"
  class:mac-layout={currentPlatform === "darwin" || currentPlatform === "macos"}
  data-tauri-drag-region
>
  {#if currentPlatform === "darwin" || currentPlatform === "macos"}
    <!-- macOS controls -->
    <div class="mac-controls">
     
    </div>
  {:else if currentPlatform === "win32" || currentPlatform === "windows"}
    <!-- Windows controls -->
    <div class="win-controls">
      <button class="win-button minimize" on:click={minimize}>&#xE921;</button>
      <button class="win-button maximize" on:click={maximizeOrUnmaximize}
        >&#xE922;</button
      >
      >
      <button class="win-button close" on:click={close}>&#xE8BB;</button>
    </div>
  {:else}
    <!-- Linux/Other controls (generic) -->
    <div class="linux-controls">
      <button class="linux-button minimize" on:click={minimize}>&#x2212;</button
      >
      <button class="linux-button maximize" on:click={maximizeOrUnmaximize}
        >&#x25A1;</button
      >
      >
      <button class="linux-button close" on:click={close}>&#x2715;</button>
    </div>
  {/if}
</div>

<style>
  .window-controls {
    display: flex;
    align-items: center;
    height: 30px; /* Adjust as needed for your header bar */
    -webkit-app-region: drag; /* Enable dragging for the entire control bar */
    position: fixed; /* Use fixed to ensure it's always at the top of the viewport */
    top: 0;
    left: 0;
    width: 100vw; /* Ensure it spans the full viewport width */
    z-index: 9999; /* Ensure it's above other content */
  }

  .window-controls.mac-layout {
    justify-content: flex-start;
    left: 0;
    padding-left: 10px;
    padding-right: 0;
  }

  .window-controls:not(.mac-layout) {
    justify-content: flex-end;
    right: 0; /* Align to right for non-macOS */
    padding-left: 0;
    padding-right: 10px;
  }

  /* macOS Controls */
  .mac-controls {
    display: flex;
    gap: 8px;
    -webkit-app-region: no-drag; /* Disable dragging on buttons */
  }

  /* Windows Controls */
  .win-controls {
    display: flex;
    -webkit-app-region: no-drag;
  }

  .win-button {
    background-color: transparent;
    border: none;
    width: 45px;
    height: 30px;
    font-family: "Segoe MDL2 Assets"; /* Windows system font for icons */
    font-size: 10px;
    cursor: pointer;
    color: #333; /* Adjust color as needed */
    display: flex;
    justify-content: center;
    align-items: center;
    transition: background-color 0.1s ease;
  }

  .win-button:hover {
    background-color: rgba(0, 0, 0, 0.1);
  }

  .win-button.close:hover {
    background-color: #e81123;
    color: white;
  }

  /* Linux Controls (Generic) */
  .linux-controls {
    display: flex;
    gap: 5px;
    -webkit-app-region: no-drag;
  }

  .linux-button {
    background-color: #eee;
    border: 1px solid #ccc;
    border-radius: 3px;
    width: 25px;
    height: 25px;
    font-size: 14px;
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: background-color 0.1s ease;
  }

  .linux-button:hover {
    background-color: #ddd;
  }
</style>
