<script lang="ts">
  import Loader from "./Loader.svelte";

  // take props from the parent component

  let {
    projectName,
    hostName,
    port,
    userName,
    password,
    dbName,
    loaderActive,
    OnClickConnect,
  }: {
    projectName: string;
    hostName: string;
    port: number;
    userName: string;
    password: string;
    dbName: string;
    loaderActive: boolean;
    OnClickConnect: (e: MouseEvent) => void;
  } = $props();

  $inspect("In ConnectionFormSvelte: ", projectName, hostName, port, userName, password, dbName, loaderActive);
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
        <label class="label" for="projectName">Project Name</label>
        <div class="control">
          <input
            bind:value={projectName}
            id="projectName"
            class="input"
            type="text"
            placeholder="Enter Project name"
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
            oninput={(e: Event) => {
              const target = e.target as HTMLInputElement;
              if (target && !isNaN(parseInt(target.value))) {
                port = parseInt(target.value);
              } else if (target) {
                target.value = port.toString();
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
        <label class="label" for="projectName">Database Name</label>
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

  <button class="button is-primary" onclick={OnClickConnect}>
    <span class="connect-btn-text"> Connect </span>
    <Loader {loaderActive} color="#1363df" />
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
    font-size: 1rem;
  }
</style>
