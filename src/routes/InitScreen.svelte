<script lang="ts">
  export const ssr = false;

  import { invoke } from "@tauri-apps/api/core";
  import { navigate } from "svelte-routing";
  import { notificationMsg } from "../stores";
  import { onMount } from "svelte";
  import {
    NOTIFICATION_TYPE_SUCCESS,
    NOTIFICATION_TYPE_ERROR,
  } from "../constants/constants";
  import ConnectionForm from "../components/ConnectionForm.svelte";
  import RecentProjects from "../components/RecentProjects.svelte";

  let projectName: string = $state("test");
  let hostName: string = $state("localhost");
  let port: number = $state(5432);
  let userName: string = $state("dev");
  let password: string = $state("1234");
  let dbName: string = $state("datasquirrel");
  let dbType: string = $state("postgres");
  let loaderActive: boolean = $state(false);
  let recentProjects: any[] = $state([
    {
      name: "Project 2",
      hostName: "localhost",
      port: 5432,
      userName: "dev",
      password: "1234",
      dbName: "datasquirrel",
      dbType: "postgres",
    },
    {
      name: "Project 3",
      hostName: "localhost",
      port: 5432,
      userName: "dev",
      password: "1234",
      dbName: "datasquirrel",
      dbType: "postgres",
    },
    {
      name: "Project 4",
      hostName: "localhost",
      port: 5432,
      userName: "dev",
      password: "1234",
      dbName: "datasquirrel",
      dbType: "postgres",
    },
  ]);

  onMount(() => {
    const savedProjects = localStorage.getItem("recentProjects");
    if (savedProjects) {
      recentProjects = JSON.parse(savedProjects);
    }
  });

  function loadRecentProject(project: any) {
    projectName = project.name;
    hostName = project.hostName;
    port = project.port;
    userName = project.userName;
    password = project.password;
    dbName = project.dbName;
  }

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
    invoke("init_connection", {
      reqPayload: {
        conn_name: projectName,
        host_name: hostName,
        database_name: dbName,
        port: parseInt(port as any),
        user_name: userName,
        password: password,
      },
    })
      .then((res: any) => {
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
  <div class="columns-2 flex w-5/6 justify-evenly">
    <div id="recent-projects-container" class="w-1/2 grid grid-flow-col text-center">
        <RecentProjects />
    </div>
    <div class="connection-form-container w-1/2">
      <ConnectionForm
        {projectName}
        {hostName}
        {port}
        {userName}
        {password}
        {dbName}
        {dbType}
        {loaderActive}
        {OnClickConnect}
      />
    </div>
  </div>
</div>

<style lang="postcss">
  @reference "tailwindcss";
  #init-connection-container {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    height: 100vh;
    width: 100%;
    min-width: 1200px;
    min-height: 1000px;
    background-color: #f0f0f0;
  }

  .recent-projects {
    margin-top: 2rem;
    width: 50%;
  }

  .recent-projects h3 {
    margin-bottom: 0.5rem;
  }

  .recent-projects ul {
    list-style: none;
    padding: 0;
  }

  .recent-projects li {
    margin-bottom: 0.5rem;
  }

  .recent-projects button {
    background: none;
    border: none;
    color: var(--color-link);
    cursor: pointer;
    text-align: left;
    width: 100%;
    padding: 0.5rem;
    border-radius: 4px;
  }

  .recent-projects button:hover {
    background-color: var(--color-bg-hover);
  }
</style>
