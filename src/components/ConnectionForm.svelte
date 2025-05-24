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
    dbType,
    loaderActive,
    OnClickConnect,
  }: {
    projectName: string;
    hostName: string;
    port: number;
    dbType: string;
    userName: string;
    password: string;
    dbName: string;
    loaderActive: boolean;
    OnClickConnect: (e: MouseEvent) => void;
  } = $props();

  $inspect(
    "In ConnectionFormSvelte: ",
    projectName,
    hostName,
    port,
    dbType,
    userName,
    password,
    dbName,
    loaderActive,
  );
</script>

<div
  class="max-w-lg border border-gray-200 rounded-lg p-4 bg-white shadow-md mx-auto"
>
  <div class="flex flex-wrap mx-3 mb-5">
    <div class="w-full px-3 my-5">
      <h2 class="text-2xl text-center">New Database Connection</h2>
    </div>
  </div>
  <div class="flex flex-wrap -mx-3">
    <div class="w-full px-3">
      <div class="mb-4">
        <label
          class="block text-gray-700 text-sm font-bold mb-2"
          for="projectName">Project Name</label
        >
        <div class="relative">
          <input
            bind:value={projectName}
            id="projectName"
            class="shadow appearance-none border border-gray-200 rounded w-full py-2 px-3 text-gray-500 leading-tight focus:outline-none focus:shadow-outline"
            type="text"
            placeholder="Enter Project name"
            autocomplete="off"
          />
        </div>
      </div>
    </div>
  </div>
  <!-- Add a option to select wshich database type like mysql postgresq  -->
  <div class="flex flex-wrap -mx-3">
    <div class="w-full px-3">
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="dbType"
          >Database Type</label
        >
        <div class="relative">
          <div class="inline-block relative w-full">
            <select
              id="dbType"
              bind:value={dbType}
              class="block appearance-none w-full bg-white border border-gray-200 hover:border-gray-400 px-4 py-2 pr-8 rounded shadow leading-tight focus:outline-none focus:shadow-outline"
            >
              <option value="postgres">PostgreSQL</option>
              <!-- <option value="mysql">MySQL</option> -->
              <!-- Add more database types as needed -->
            </select>
            <div
              class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700"
            >
              <svg
                class="fill-current h-4 w-4"
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 20 20"
                ><path
                  d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
                /></svg
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="flex flex-wrap -mx-3">
    <div class="w-full md:w-1/2 px-3">
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="hostName"
          >Host</label
        >
        <div class="relative">
          <input
            id="hostName"
            class="shadow appearance-none border border-gray-200 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            type="text"
            bind:value={hostName}
            placeholder="Enter Host"
            autocomplete="off"
          />
        </div>
      </div>
    </div>

    <div class="w-full md:w-1/2 px-3">
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="port"
          >Port</label
        >
        <div class="relative">
          <input
            id="port"
            class="shadow appearance-none border border-gray-200 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
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

  <div class="flex flex-wrap -mx-3">
    <div class="w-full md:w-1/2 px-3">
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="userName"
          >Username</label
        >
        <div class="relative">
          <input
            id="userName"
            class="shadow appearance-none border border-gray-200 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            type="text"
            bind:value={userName}
            placeholder="Enter Username"
            autocomplete="off"
          />
        </div>
      </div>
    </div>

    <div class="w-full md:w-1/2 px-3">
      <div class="mb-4">
        <label class="block text-gray-700 text-sm font-bold mb-2" for="password"
          >Password</label
        >
        <div class="relative">
          <input
            id="password"
            class="shadow appearance-none border border-gray-200 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            type="password"
            bind:value={password}
            placeholder="Enter Password"
            autocomplete="off"
          />
        </div>
      </div>
    </div>
  </div>

  <div class="flex flex-wrap -mx-3">
    <div class="w-full px-3">
      <div class="mb-4">
        <label
          class="block text-gray-700 text-sm font-bold mb-2"
          for="projectName">Database Name</label
        >
        <div class="relative">
          <input
            id="dbName"
            class="shadow appearance-none border border-gray-200 rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
            type="text"
            bind:value={dbName}
            placeholder="Enter database name"
            autocomplete="off"
          />
        </div>
      </div>
    </div>
  </div>

  <button
    class="bg-blue-500 hover:bg-blue-700 text-white font-bold w-35 h-10 rounded focus:outline-none focus:shadow-outline flex items-center justify-center gap-2"
    onclick={OnClickConnect}
    disabled={loaderActive}
  >
    {#if !loaderActive}
      <span class="font-semibold text-base"> Connect </span>
    {/if}
    <Loader loaderActive={loaderActive} color="#1363df" />
  </button>
</div>
