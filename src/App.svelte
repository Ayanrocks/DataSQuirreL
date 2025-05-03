<script>
    export const ssr = false;
    import { Router, Route } from 'svelte-routing';
    import InitConnection from './routes/InitConnection.svelte';
    import MainScreen from './routes/MainScreen.svelte';
    import Notification from './components/Notification.svelte';
</script>

<Notification/>
<Router>
    <div id="router-view">
        <Route path="/">
            <InitConnection/>
        </Route>
        <Route path="/dashboard">
            <MainScreen/>
        </Route>
    </div>
</Router>


<style>
    #router-view {
        overflow: hidden;
    }
</style>
