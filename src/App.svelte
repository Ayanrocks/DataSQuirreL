<script>
  import { Router, Link, Route } from 'svelte-navigator';
  import InitConnection from './views/initConnection.svelte';
  import MainScreen from './views/mainScreen.svelte';
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
