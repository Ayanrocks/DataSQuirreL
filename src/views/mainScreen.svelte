<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import { useFocus } from 'svelte-navigator';

  const registerFocus = useFocus();

  onMount(() => {
    console.log('OnLoad');
    invoke('fetch_tables')
      .then((res) => {
        console.log(res);
        if (res.error_code) {
          notificationMsg.set({ type: NOTIFICATION_TYPE_ERROR, message: res.frontend_msg });
          return;
        }
      })
      .catch((e) => {
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: 'Something went wrong. Check console for more information',
        });
      });
  });
</script>

<div class="main-container" use:registerFocus>
  <div class="columns split-view-container">
    <div class="column is-one-quarter split-sidebar">is-four-fifths</div>
    <div class="column split-main-content">Auto</div>
  </div>
</div>

<style>
  .main-container {
    height: 100vh;
    width: 100vw;
  }

  .split-view-container {
    height: 100%;
    width: 100%;
  }

  .split-sidebar {
    position: absolute;
    width: 96px;
    height: 100%;
    left: 0;
    background-color: #f0f0ff;
    min-width: 250px;
  }

  .split-sidebar::after {
    content: '';
    background-color: rgb(161, 161, 161);
    position: absolute;
    right: 0;
    width: 4px;
    height: 98%;
    cursor: ew-resize;
  }
</style>
