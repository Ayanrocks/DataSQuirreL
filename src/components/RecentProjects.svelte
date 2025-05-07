<script lang="ts">
  // Placeholder data for recent projects
  const recentProjects = [
    { id: 1, name: "Project Alpha" },
    { id: 2, name: "Project Beta" },
    { id: 3, name: "Project Gamma" },
  ];
</script>

<div class="recent-projects w-full py-25">
  <div class="recent-projects-header w-full">
    <h2 class="text-3xl w-full">Recent Projects</h2>
  </div>

  <div class="col-start-1 my-25">
    {#if recentProjects.length > 0}
      <ul class="list-none p-0 m-0 w-full ">
        {#each recentProjects as project}
          <li
            class="p-2 mb-1 border border-gray-200 rounded-md transition duration-300 ease-in-out cursor-pointer hover:shadow-md"
          >
            {project.name}
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

<style>
  /* No component-specific styles needed with Tailwind */
</style>
