<script lang="ts">
  import settingsIcon from "../assets/icons/settings.svg?raw";
  import AboutModal from "./AboutModal.svelte";

  let { connectionName }: { connectionName: string } = $props();

  let isAboutOpen = $state(false);

  function openAbout() {
    isAboutOpen = true;
  }

  function closeAbout() {
    isAboutOpen = false;
  }
</script>

<div class="top-bar_container">
  <div class="top-bar_left">
    <div class="top-bar_left_title">{connectionName} - DataSquirrel</div>
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="top-bar_right" onclick={openAbout}>
    <div class="top-bar_right_settings-icon">
      {@html settingsIcon}
    </div>
  </div>
</div>

<AboutModal isOpen={isAboutOpen} onClose={closeAbout} />

<style>
  .top-bar_container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px;
    font-family: "Outfit", sans-serif;
    background-color: #fff4f4;
    border-bottom: 1px solid #e5e5e5;
    height: 50px;
    width: 100%;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1000;
    box-shadow: 0 0 2px 0 rgba(0, 0, 0, 0.1);
  }

  .top-bar_left {
    display: flex;
    align-items: center;
    margin-left: 100px;
  }

  .top-bar_left_title {
    font-size: 18px;
    font-weight: 600;
    color: #000;
    font-weight: 400;
  }

  .top-bar_right {
    display: flex;
    align-items: center;
    margin-right: 20px;
  }

  .top-bar_right_settings-icon {
    cursor: pointer;
    color: #777777;
  }
</style>
