<script>
  import { Router, Link, Route } from 'svelte-navigator';
  import InitConnection from './views/InitConnection.svelte';
  import MainScreen from './views/MainScreen.svelte';
  import Notification from './components/Notification.svelte';
</script>

<Notification />
<Router>
  <div id="router-view">
    <Route path="/">
      <InitConnection />
    </Route>
    <Route path="dashboard" component={MainScreen} />
  </div>
</Router>

<style>
  #router-view {
    overflow: hidden;
  }
</style>
