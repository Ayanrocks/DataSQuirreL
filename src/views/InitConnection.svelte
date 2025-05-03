<script>
    // With the Tauri API npm package:
    import {invoke} from "@tauri-apps/api/core";
    import {navigate} from "svelte-routing";
    import {notificationMsg} from "../stores";
    import {
        NOTIFICATION_TYPE_SUCCESS,
        NOTIFICATION_TYPE_ERROR,
    } from "../constants/constants";
    import Loader from "../components/Loader.svelte";

    // Reactive variables
    let connName = "test";
    let hostName = "localhost";
    let port = 5432;
    let userName = "dev";
    let password = "1234";
    let dbName = "datasquirrel";
    let loaderActive = false;

    function OnClickConnect(e) {
        if (
            connName === "" ||
            hostName === "" ||
            port === 0 ||
            userName === "" ||
            password === ""
        ) {
            alert("Please enter all details");
            return;
        }
        loaderActive = true;
        // Invoke the command
        invoke("init_connection", {
            reqPayload: {
                conn_name: connName,
                host_name: hostName,
                database_name: dbName,
                port: parseInt(port),
                user_name: userName,
                password: password,
            },
        })
            .then((res) => {
                loaderActive = false;
                console.log(res);
                if (res.error_code) {
                    notificationMsg.set({
                        type: NOTIFICATION_TYPE_ERROR,
                        message: res.frontend_msg,
                    });
                    return;
                }

                notificationMsg.set({
                    type: NOTIFICATION_TYPE_SUCCESS,
                    message: res.frontend_msg,
                });

                setTimeout(() => {
                    navigate("/dashboard");
                }, 500);
            })
            .catch((e) => {
                loaderActive = false;
                console.log(e);
                notificationMsg.set({
                    type: NOTIFICATION_TYPE_ERROR,
                    message: "Something went wrong. Check console for more information",
                });
            });
    }
</script>

<div class="form-container">
    <div class="columns">
        <div class="column is-full">
            <h2 class="is-size-4 has-text-centered">New Database Connection</h2>
        </div>
    </div>
    <div class="columns">
        <div class="column is-full">
            <div class="field">
                <label class="label" for="connName">Connection Name</label>
                <div class="control">
                    <input
                            bind:value={connName}
                            id="connName"
                            class="input"
                            type="text"
                            placeholder="Enter connection name"
                            autocomplete="off"
                    />
                </div>
            </div>
        </div>
    </div>
    <div class="columns">
        <div class="column is-half">
            <div class="field">
                <label class="label" for="hostName">Host</label>
                <div class="control">
                    <input
                            id="hostName"
                            class="input"
                            type="text"
                            bind:value={hostName}
                            placeholder="Enter Host"
                            autocomplete="off"
                    />
                </div>
            </div>
        </div>

        <div class="column is-half">
            <div class="field">
                <label class="label" for="port">Port</label>
                <div class="control">
                    <input
                            id="port"
                            class="input"
                            type="text"
                            value={port}
                            on:input={e => {
                                if(!isNaN(e.target.value)) {
                                  port = e.target.value
                                } else {
                                  e.target.value = port
                                }
                            }}
                            placeholder="Enter Port"
                            autocomplete="off"
                    />
                </div>
            </div>
        </div>
    </div>

    <div class="columns">
        <div class="column is-half">
            <div class="field">
                <label class="label" for="userName">Username</label>
                <div class="control">
                    <input
                            id="userName"
                            class="input"
                            type="text"
                            bind:value={userName}
                            placeholder="Enter Username"
                            autocomplete="off"
                    />
                </div>
            </div>
        </div>

        <div class="column is-half">
            <div class="field">
                <label class="label" for="password">Password</label>
                <div class="control">
                    <input
                            id="password"
                            class="input"
                            type="password"
                            bind:value={password}
                            placeholder="Enter Password"
                            autocomplete="off"
                    />
                </div>
            </div>
        </div>
    </div>

    <div class="columns">
        <div class="column is-full">
            <div class="field">
                <label class="label" for="connName">Database Name</label>
                <div class="control">
                    <input
                            id="dbName"
                            class="input"
                            type="text"
                            bind:value={dbName}
                            placeholder="Enter database name"
                            autocomplete="off"
                    />
                </div>
            </div>
        </div>
    </div>

    <button class="button is-primary" on:click={OnClickConnect}>
        <span class="connect-btn-text"> Connect </span>
        <Loader {loaderActive} color="#1363df"/>
    </button>
</div>

<style>
    .form-container {
        width: 40%;
        margin: 5rem auto;
        border: 1px solid #e9e9e9;
        border-radius: 7px;
        perspective: 1px;
        padding: 1rem;
    }

    .connect-btn-text {
        font-weight: 600;
        text-transform: none;
        font-size: 1.2rem;
    }
</style>
