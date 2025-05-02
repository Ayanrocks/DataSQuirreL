<script>
    // With the Tauri API npm package:
    import {invoke} from "@tauri-apps/api/core";
    import {navigate} from "svelte-navigator";
    import {notificationMsg} from "../stores";
    import {
        NOTIFICATION_TYPE_SUCCESS,
        NOTIFICATION_TYPE_ERROR,
    } from "../constants/constants";
    import Loader from "../components/Loader.svelte";

  import { replace } from 'svelte-spa-router';
  import { type IPCResponse } from '../types/interface.ts';
  import { notificationMsg } from '../stores.ts';
  import { NOTIFICATION_TYPE_SUCCESS, NOTIFICATION_TYPE_ERROR } from '../constants/constants';
  import Loader from '../components/Loader.svelte';

  // Reactive variables
  // TODO: Set this to default empty strings
  let connName: string = 'test';
  let hostName: string = 'localhost';
  let port: number = 5432;
  let userName: string = 'dev';
  let password: string = 'password';
  let dbName: string = 'multipl-local';
  let loaderActive: boolean = false;

  /// OnClickConnect is the function that is called when the user clicks on the connect button
  async function OnClickConnect() {
    // check if fields are empty
    if (connName === '' || hostName === '' || port === 0 || userName === '' || password === '') {
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: 'Please fill all the fields',
      });
      return;
    }
    // setting the loader to active
    loaderActive = true;
    try {
      // send ipc request to rust backend
      const res: IPCResponse = await invoke('init_connection', {
        reqPayload: {
          conn_name: connName,
          host_name: hostName,
          database_name: dbName,
          port: port,
          user_name: userName,
          password: password,
        },
      });

      // if there is an error, show the error message
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

      loaderActive = false;

      // if all well, then navigate to the dashboard
      setTimeout(() => {
        replace('/dashboard');
      }, 500);
    } catch (err) {
      // if there is an error, show the error message
      loaderActive = false;
      console.log(err);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: 'Something went wrong. Check console for more information',
      });
    }
  }

  // validate if the user entered value is a number or not, else set the value to the previous value
  const validatePort = (
    e: Event & {
      currentTarget: EventTarget & HTMLInputElement;
    },
  ) => {
    let inputValue = parseInt(e.currentTarget.value);
    if (!isNaN(inputValue)) {
      port = inputValue;
    } else {
      inputValue = port;
    }
  };
</script>

<div class="container">
  <div class="form-container container grid grid-cols-1 w-full">
    <div class="columns place-content-center w-full text-2xl my-8">
      <div class="heading-label">
        <h2 class="text-center font-light">New Database Connection</h2>
      </div>
    </div>
    <div class="columns my-4">
      <div class="field">
        <label class="label font-bold" for="connName">Connection Name</label>
        <div class="control my-2">
          <input
            bind:value={connName}
            id="connName"
            class="input input-bordered border-solid border-2 w-full p-2 rounded-md"
            type="text"
            placeholder="Enter connection name"
            autocomplete="off"
          />
        </div>
      </div>
    </div>
    <div class="columns-2">
      <div class="field">
        <label class="label font-bold" for="hostName">Host</label>
        <div class="control my-2">
          <input
            id="hostName"
            class="input input-bordered border-solid border-2 w-full p-2 rounded-md"
            type="text"
            bind:value={hostName}
            placeholder="Enter Host"
            autocomplete="off"
          />
        </div>
      </div>

      <div class="field">
        <label class="label font-bold" for="port">Port</label>
        <div class="control my-2">
          <input
            id="port"
            class="input input-bordered border-solid border-2 w-full p-2 rounded-md"
            type="text"
            value={port}
            on:input={validatePort}
            placeholder="Enter Port"
            autocomplete="off"
          />
        </div>
      </div>
    </div>

    <div class="columns-2 my-4">
      <div class="field">
        <label class="label font-bold" for="userName">Username</label>
        <div class="control my-2">
          <input
            id="userName"
            class="input input-bordered border-solid border-2 w-full p-2 rounded-md"
            type="text"
            bind:value={userName}
            placeholder="Enter Username"
            autocomplete="off"
          />
        </div>
      </div>

      <div class="field">
        <label class="label font-bold" for="password">Password</label>
        <div class="control my-2">
          <input
            id="password"
            class="input input-bordered border-solid border-2 w-full p-2 rounded-md"
            type="password"
            bind:value={password}
            placeholder="Enter Password"
            autocomplete="off"
          />
        </div>
      </div>
    </div>

    <div class="columns my-2">
      <div class="field">
        <label class="label font-bold" for="dbName">Database Name</label>
        <div class="control my-2">
          <input
            id="dbName"
            class="input input-bordered border-solid border-2 w-full p-2 rounded-md"
            type="text"
            bind:value={dbName}
            placeholder="Enter database name"
            autocomplete="off"
          />
        </div>
      </div>
    </div>

    <button class="btn connect-button" on:click={OnClickConnect}>
      <span class="connect-btn-text"> Connect </span>
      <Loader {loaderActive} color="#dff6ff" />
    </button>
  </div>
</div>

<style>
  .form-container {
    width: 40%;
    margin: 5rem auto;
    border: 1px solid #e9e9e9;
    border-radius: 7px;
    perspective: 1px;
    padding: 1rem;
    height: 100%;
  }

  .connect-button {
    background-color: var(--primaryColor);
    color: #e9e9e9;
  }

  .connect-button:hover {
    background-color: var(--primaryColor);
    color: #e9e9e9;
  }
  .connect-btn-text {
    font-weight: 600;
    text-transform: none;
  }
</style>
