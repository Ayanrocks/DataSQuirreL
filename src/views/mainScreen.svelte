<script>
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  onMount(() => {
    console.log('OnLoad');
    invoke('fetch_tables').then((res) => {
      console.log(res);
    });
  });
</script>

<h2>Database Viewer</h2>

<style>
</style>
