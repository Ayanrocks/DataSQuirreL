<script lang="ts">
  import connectIcon from "../assets/icons/connect.svg?raw";
  import deleteIcon from "../assets/icons/delete.svg?raw";
  import editIcon from "../assets/icons/edit.svg?raw";

  // Placeholder data for recent projects
  const recentProjects = [
    { id: 1, name: "Project Alpha" },
    { id: 2, name: "Project Beta" },
    { id: 3, name: "Project Gamma" },
    { id: 4, name: "Project Opus" },
    { id: 5, name: "Project Delta" },
    { id: 6, name: "Project Iowa" },
    { id: 7, name: "Project Foobar" },
    { id: 8, name: "Project Hanoi" },
  ];
</script>

<div class="recent-projects w-full">
  <div class="">
    <div class="recent-projects-header w-full my-5">
      <h2 class="text-2xl w-full">Recent Projects</h2>
    </div>

    <div class="col-start-1 my-5 recent-projects-list">
      {#if recentProjects.length > 0}
        <ul class="list-none p-0 m-0 w-full overflow-scroll h-120">
          {#each recentProjects as project}
            <li
              class="p-5 mb-5 bg-[#fff] border border-gray-200 rounded-md transition duration-300 ease-in-out cursor-pointer hover:shadow-md shadow-sm flex flex-column items-center justify-between hover:bg-gray-50"
            >
              <div class="project-name">
                {project.name}
              </div>
              <div class="icon-container flex">
                <div class="connectIcon-container text-gray-500 hover:text-blue-700" title="Connect to Project">
                  {@html connectIcon}
                </div>
                <div
                  class="ml-2 text-gray-500 hover:text-blue-500 cursor-pointer"
                  title="Edit Project"
                >
                  {@html editIcon}
                </div>
                <div
                  class="ml-2 text-gray-500 hover:text-red-500 cursor-pointer"
                  title="Delete Project"
                >
                  {@html deleteIcon}
                </div>
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
