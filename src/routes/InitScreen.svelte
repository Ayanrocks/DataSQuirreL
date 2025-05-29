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
  import InitWaveSVG from "../assets/InitWave.svg?raw";
  import InitScreenEclipse from "../assets/InitScreenEclipse.svg?raw";
  import MainLogo from "../assets/MainLogo.svg?raw";
  import type { RecentProjects as RecentProjectsType } from "../types/props";

  let projectName: string = $state("test");
  let hostName: string = $state("localhost");
  let port: number = $state(5432);
  let userName: string = $state("dev");
  let password: string = $state("1234");
  let dbName: string = $state("datasquirrel");
  let dbType: string = $state("postgres");
  let connectionFormConnectLoader: boolean = $state(false);
  let recentProjectsLoader: boolean = $state(false);
  let recentProjects: RecentProjectsType[] = $state([
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
    connectionFormConnectLoader = true;
    connect(connectionFormConnectLoader);
  }

  function connect(_loaderActive: boolean) {
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
        switch (_loaderActive) {
          case connectionFormConnectLoader:
            connectionFormConnectLoader = false;
            break;
          case recentProjectsLoader:
            recentProjectsLoader = false;
            break;
        }
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
        switch (_loaderActive) {
          case connectionFormConnectLoader:
            connectionFormConnectLoader = false;
            break;
          case recentProjectsLoader:
            recentProjectsLoader = false;
            break;
        }
        console.log(e);
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: "Something went wrong. Check console for more information",
        });
      });
  }

  function onConnectRecentProject(project: any) {
    projectName = project.name;
    hostName = project.hostName;
    port = project.port;
    userName = project.userName;
    password = project.password;
    dbName = project.dbName;
    recentProjectsLoader = true;
    connect(recentProjectsLoader);
  }
</script>

<div id="init-connection-container">
  <div class="main-logo-container" title="DataSquirrel">
    {@html MainLogo}
  </div>
  <div class="columns-2 flex w-5/6 justify-evenly z-1 mt-20">
    <div
      id="recent-projects-container"
      class="w-1/2 grid grid-flow-col text-center"
    >
      <RecentProjects
        projects={recentProjects}
        onConnect={onConnectRecentProject}
        onEdit={() => {}}
        onDelete={() => {}}
        recentProjectsLoading={recentProjectsLoader}
      />
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
        loaderActive={connectionFormConnectLoader}
        {OnClickConnect}
      />
    </div>
  </div>
  <div class="init-connection-container--top-eclipse absolute">
    {@html InitScreenEclipse}
  </div>
  <div class="init-connection-container--background absolute">
    {@html InitWaveSVG}
  </div>
</div>

<style lang="postcss">
  @reference "tailwindcss";

  .main-logo-container {
    position: absolute;
    top: -3%;
    left: 50%;
    transform: translateX(-50%);
    z-index: 10;
  }

  #init-connection-container {
    font-size: 0.9rem;
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    height: 100vh;
    width: 100%;
    min-width: 1080px;
    min-height: 900px;
    background-color: #f0f0f0;
    position: relative;
  }

  .init-connection-container--background {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    bottom: 10%;
    width: 100%;
    height: 300px;
    background-size: cover;
    background-repeat: no-repeat;
  }

  .init-connection-container--top-eclipse {
    position: absolute;
    top: -5%;
    left: -5%;
    right: 0;
    width: 100%;
    height: 100%;
    background-size: cover;
    background-repeat: no-repeat;
  }
</style>
