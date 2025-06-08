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
  import { v4 as uuidv4 } from "uuid";
  import type { IPCResponse, StoredConnection } from "../types/response";

  let projectId = $state("");
  let projectName = $state("test");
  let hostName = $state("localhost");
  let port = $state(5432);
  let userName = $state("dev");
  let password = $state("1234");
  let dbName = $state("datasquirrel");
  let dbType = $state("postgres");
  let connectionFormConnectLoader = $state(false);
  let recentProjectsLoader = $state(false);
  let recentProjects = $state<RecentProjectsType[]>([]);

  // Function to check if a project with the same connection details already exists
  function findExistingProjectId(
    project: Omit<RecentProjectsType, "id">,
  ): string | undefined {
    return recentProjects.find(
      (p) =>
        p.hostName === project.hostName &&
        p.port === project.port &&
        p.userName === project.userName &&
        p.dbName === project.dbName &&
        p.dbType === project.dbType,
    )?.id;
  }

  onMount(() => {
    loadRecentProjects();
  });

  function OnClickConnect(e: MouseEvent) {
    if (
      projectName === "" ||
      hostName === "" ||
      port === 0 ||
      userName === "" ||
      password === ""
    ) {
      console.log(
        "not all fields are filled",
        projectName,
        hostName,
        port,
        userName,
        password,
      );
      return;
    }
    connectionFormConnectLoader = true;
    connect(connectionFormConnectLoader);
  }

  function connect(_loaderActive: boolean) {
    // if projectId is empty then create a new project
    if (projectId === "") {
      projectId = uuidv4();
    }
    invoke("init_connection", {
      reqPayload: {
        id: projectId,
        conn_name: projectName,
        host_name: hostName,
        database_name: dbName,
        database_type: dbType,
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
        resetState();
        // Refresh the recent projects list
        loadRecentProjects();
      })
      .catch((err) => {
        console.error(err);
        switch (_loaderActive) {
          case connectionFormConnectLoader:
            connectionFormConnectLoader = false;
            break;
          case recentProjectsLoader:
            recentProjectsLoader = false;
            break;
        }
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: "Failed to connect to database",
        });
      });
  }

  function resetState() {
    // reset state for all project variables
    projectId = "";
    projectName = "";
    hostName = "";
    port = 0;
    userName = "";
    password = "";
    dbName = "";
    dbType = "";
    connectionFormConnectLoader = false;
    recentProjectsLoader = false;
  }

  async function loadRecentProjects() {
    recentProjectsLoader = true;
    try {
      const res = await invoke<IPCResponse<StoredConnection[]>>(
        "get_saved_connections",
      );

      // Map StoredConnection to RecentProjectsType
      recentProjects = (res.data || []).map((conn) => ({
        id: conn.id,
        name: conn.conn_name,
        hostName: conn.host_name,
        port: conn.port,
        userName: conn.user_name,
        password: "", // Password is not stored in the frontend
        dbName: conn.database_name,
        dbType: conn.database_type, // Default to postgres for now
      }));
    } catch (err) {
      console.error(err);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "Failed to load recent projects",
      });
    } finally {
      recentProjectsLoader = false;
    }
  }

  async function deleteProject(project: RecentProjectsType) {
    try {
      const res = await invoke<IPCResponse<string>>("delete_saved_connection", {
        connName: project.name,
        projectId: project.id,
      });
      if (res.error_code) {
        notificationMsg.set({
          type: NOTIFICATION_TYPE_ERROR,
          message: res.frontend_msg || "Failed to delete project",
        });
        return;
      }
      resetState();

      notificationMsg.set({
        type: NOTIFICATION_TYPE_SUCCESS,
        message: res.frontend_msg || "Project deleted successfully",
      });
    } catch (err) {
      console.error(err);
      notificationMsg.set({
        type: NOTIFICATION_TYPE_ERROR,
        message: "Failed to delete project",
      });
    }
    // Refresh the recent projects list
    loadRecentProjects();
  }

  function onConnectRecentProject(project: RecentProjectsType) {
    projectId = project.id || "";
    projectName = project.name;
    hostName = project.hostName;
    port = project.port;
    dbType = project.dbType;
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
        onDelete={deleteProject}
        recentProjectsLoading={recentProjectsLoader}
      />
    </div>
    <div class="connection-form-container w-1/2">
      <ConnectionForm
        bind:projectName
        bind:hostName
        bind:port
        bind:userName
        bind:password
        bind:dbName
        bind:dbType
        bind:loaderActive={connectionFormConnectLoader}
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
