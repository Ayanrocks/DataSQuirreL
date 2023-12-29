<script>
  import Router from 'svelte-spa-router';
  import InitConnection from './views/InitConnection.svelte';
  import MainScreen from "./views/MainScreen.svelte";
  import Notification from './components/Notification.svelte';
  import './tailwind.css';

  const routes = {
    '/': InitConnection,
    '/dashboard': MainScreen,
  };
</script>

<Notification />
<div id="router-view" class="container">
  <Router {routes} />
</div>

<style>
  #router-view {
    overflow: hidden;
  }
</style>
