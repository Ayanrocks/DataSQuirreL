<script>
  export const ssr = false;
  import { Router, Route } from "svelte-routing";
  import InitScreen from "./routes/InitScreen.svelte";
  import MainScreen from "./routes/MainScreen.svelte";
  import Notification from "./components/Notification.svelte";
  import "./App.css";
</script>

<Notification />
<Router>
  <div id="router-view">
    <Route path="/">
      <InitScreen />
    </Route>
    <Route path="/dashboard">
      <MainScreen />
    </Route>
  </div>
</Router>

<style>
  #router-view {
    overflow: hidden;
    height: 100%;
    width: 100%;
  }
</style>
