<script>
  // With the Tauri API npm package:
  import { invoke } from '@tauri-apps/api/tauri';

  let connName = '';
  let hostName = '';
  let port = 0;
  let userName = '';
  let password = '';

  function OnClick(e) {
    console.log('Passowrd: ', connName, password, userName);
    if (connName == '' || hostName == '' || port == 0 || userName == '' || password == '') {
      alert('Please enter all details');
      return;
    }
    // Invoke the command
    invoke('my_custom_command', {
      reqPayload: {
        conn_name: connName,
        host_name: hostName,
        port: parseInt(port),
        user_name: userName,
      },
    }).catch((e) => {
      console.log(e);
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
          <input id="connName" class="input" type="text" bind:value={connName} placeholder="Enter connection name" />
        </div>
      </div>
    </div>
  </div>
  <div class="columns">
    <div class="column is-half">
      <div class="field">
        <label class="label" for="hostName">Host</label>
        <div class="control">
          <input id="hostName" class="input" type="text" bind:value={hostName} placeholder="Enter Host" />
        </div>
      </div>
    </div>

    <div class="column is-half">
      <div class="field">
        <label class="label" for="port">Port</label>
        <div class="control">
          <input id="port" class="input" type="number" bind:value={port} placeholder="Enter Port" />
        </div>
      </div>
    </div>
  </div>

  <div class="columns">
    <div class="column is-half">
      <div class="field">
        <label class="label" for="userName">Username</label>
        <div class="control">
          <input id="userName" class="input" type="text" bind:value={userName} placeholder="Enter Username" />
        </div>
      </div>
    </div>

    <div class="column is-half">
      <div class="field">
        <label class="label" for="password">Password</label>
        <div class="control">
          <input id="password" class="input" type="password" bind:value={password} placeholder="Enter Password" />
        </div>
      </div>
    </div>
  </div>

  <!-- <div class="buttons" on:click={OnClick}> -->
  <button class="button is-primary" on:click={OnClick}>
    <span class="connect-btn-text"> Connect </span>
  </button>
  <!-- </div> -->
</div>

<style>
  .form-container {
    width: 40%;
    margin: 10rem auto;
    border: 1px solid #e9e9e9;
    border-radius: 7px;
    perspective: 1px;
    padding: 1rem;
  }

  .connect-btn-text {
    font-weight: 'semibold';
    text-transform: none;
    font-size: 1.2rem;
  }
</style>
