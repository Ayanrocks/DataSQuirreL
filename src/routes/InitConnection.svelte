<script lang="ts">
  export const ssr = false;

  // With the Tauri API npm package:
  import { invoke } from "@tauri-apps/api/core";
  import { navigate } from "svelte-routing";
  import { notificationMsg } from "../stores";
  import {
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_ERROR,
  } from "../constants/constants";
  import ConnectionForm from "../components/ConnectionForm.svelte";

  // Reactive variables
  let projectName: string = "test";
  let hostName: string = "localhost";
  let port: number = 5432;
  let userName: string = "dev";
  let password: string = "1234";
  let dbName: string = "datasquirrel";
  let loaderActive: boolean = $state(false);

  function OnClickConnect(e: MouseEvent) {
    if (
      projectName === "" ||
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
        conn_name: projectName,
        host_name: hostName,
        database_name: dbName,
        port: parseInt(port as any), // TODO: Fix this type assertion if possible
        user_name: userName,
        password: password,
      },
    })
      .then((res: any) => {
        // TODO: Define a proper type for res
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
          navigate("/dashboard", { replace: true });
        }, 500);
      })
      .catch((e: any) => {
        // TODO: Define a proper type for e
        loaderActive = false;
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: "Something went wrong. Check console for more information",
        });
      });
  }
</script>

<div id="init-connection-container">
  <!-- Connection Form -->
  <ConnectionForm
    {projectName}
    {hostName}
    {port}
    {userName}
    {password}
    {dbName}
    {loaderActive}
    {OnClickConnect}
  />
</div>

<style>
  #init-connection-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    width: 100vw;
    background-color: #f0f0f0;
  }
</style>
