<script>
    import { Router, Route } from 'svelte-routing';
    import InitConnection from './views/InitConnection.svelte';
    import MainScreen from './views/MainScreen.svelte';
    import Notification from './components/Notification.svelte';
</script>

<Notification/>
<Router>
    <div id="router-view">
        <Route path="/">
            <InitConnection/>
        </Route>
        <Route path="dashboard">
            <MainScreen/>
        </Route>
    </div>
</Router>

<style>
    #router-view {
        overflow: hidden;
    }
</style>
