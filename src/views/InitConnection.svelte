<script lang="ts">
  // With the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/tauri";
  import { notificationMsg } from "../stores";
  import {
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_ERROR,
  } from "../constants/constants";
  import Loader from "../components/Loader.svelte";

  // Reactive variables
  // TODO: Set this to default empty strings
  let connName: String = "test";
  let hostName: String = "localhost";
  let port: Number = 5432;
  let userName: String = "dev";
  let password: String = "password";
  let dbName: String = "multipl-local";
  let loaderActive: boolean = true;

  function OnClickConnect(_: any) {
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
        port: port,
        user_name: userName,
        password: password,
      },
    })
      .then((res: { error_code: any; frontend_msg: any }) => {
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
          // navigate("/dashboard");
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

  const validatePort = (e: HTMLInputElement) => {
    if (e.target !== null) {
      if (!isNaN(e.target.value)) {
        port = e.target.value;
      } else {
        e.target.value = port;
      }
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
            on:input={(e) => {
              validatePort(e);
            }}
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
