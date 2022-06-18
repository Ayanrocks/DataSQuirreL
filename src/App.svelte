<script>
  import { Router, Link, Route } from 'svelte-navigator';
  import InitConnection from './views/initConnection.svelte';
  import Notification from './components/Notification.svelte';
</script>

<Notification />
<Router>
  <div id="router-view">
    <Route path="/">
      <InitConnection />
    </Route>
  </div>
</Router>

<style>
  #router-view {
    overflow: hidden;
  }
</style>
