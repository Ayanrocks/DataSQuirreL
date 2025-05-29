<script lang="ts">
  import connectIcon from "../assets/icons/connect.svg?raw";
  import deleteIcon from "../assets/icons/delete.svg?raw";
  import editIcon from "../assets/icons/edit.svg?raw";
  import folderIcon from "../assets/icons/folder.svg?raw";
  import type { RecentProjects as RecentProjectsType } from "../types/props";
  import Loader from "./Loader.svelte";

  // Get props for recent projects and handlers
  const { projects, onConnect, onEdit, onDelete, recentProjectsLoading } =
    $props<{
      projects: Array<RecentProjectsType>;
      onConnect: (project: RecentProjectsType) => void;
      onEdit: (project: RecentProjectsType) => void;
      onDelete: (project: RecentProjectsType) => void;
      recentProjectsLoading: boolean;
    }>();

  // Add state to track which project is being connected
  let connectingProjectName = $state<string | null>(null);

  // Modify the connect handler
  function handleConnect(project: RecentProjectsType) {
    connectingProjectName = project.name;
    onConnect(project);
  }
</script>

<div class="recent-projects w-full">
  <div class="">
    <div class="recent-projects-header w-full my-5">
      <h2 class="text-2xl w-full">Recent Projects</h2>
    </div>

    <div class="col-start-1 my-5 recent-projects-list">
      {#if projects.length > 0}
        <ul class="list-none p-0 m-0 w-full overflow-scroll h-120">
          {#each projects as project}
            <li
              class="p-5 mb-5 bg-[#fff] border border-gray-200 rounded-md transition duration-300 ease-in-out cursor-pointer hover:shadow-md shadow-sm flex flex-column items-center justify-between hover:bg-gray-50"
            >
              <div class="project-name flex items-center gap-2">
                <span class="icon-container scale-75">
                  {@html folderIcon}
                </span>
                {project.name}
              </div>
              <div class="icon-container flex items-center">
                <div class="loader-container mr-2 scale-75 h-5 w-5">
                  <Loader
                    loaderActive={recentProjectsLoading && connectingProjectName === project.name}
                    color="#1363df"
                  />
                </div>
                <button
                  type="button"
                  class="connectIcon-container text-gray-500 hover:text-green-400 scale-75 cursor-pointer bg-transparent border-none"
                  title="Connect to Project"
                  onclick={() => handleConnect(project)}
                  onkeydown={(e) => e.key === "Enter" && handleConnect(project)}
                  disabled={recentProjectsLoading}
                >
                  {@html connectIcon}
                </button>
                <button
                  type="button"
                  class="ml-2 text-gray-500 hover:text-blue-400 cursor-pointer scale-75 bg-transparent border-none"
                  title="Edit Project"
                  onclick={() => onEdit(project)}
                  onkeydown={(e) => e.key === "Enter" && onEdit(project)}
                  disabled={recentProjectsLoading}
                >
                  {@html editIcon}
                </button>
                <button
                  type="button"
                  class="ml-2 text-gray-500 hover:text-red-400 cursor-pointer scale-75 bg-transparent border-none"
                  title="Delete Project"
                  onclick={() => onDelete(project)}
                  onkeydown={(e) => e.key === "Enter" && onDelete(project)}
                  disabled={recentProjectsLoading}
                >
                  {@html deleteIcon}
                </button>
              </div>
            </li>
          {/each}
        </ul>
      {:else}
        <div class="flex items-center justify-center h-32 text-gray-500">
          No recent Project
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  /* No component-specific styles needed with Tailwind */
  .recent-projects {
    width: 100%;
  }

  .recent-projects ul {
    list-style: none;
    padding: 0;
  }

  .recent-projects li {
    margin-bottom: 1rem;
  }

  .recent-projects-header {
    margin-bottom: 0.5rem;
  }

  .recent-projects-list {
    max-height: 30rem;
    overflow-y: auto;
    width: 80%;
    margin: 0 auto;
  }
</style>
