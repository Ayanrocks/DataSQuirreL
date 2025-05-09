<script lang="ts">
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

<div class="recent-projects w-full py-25">
  <div class="recent-projects-header w-full">
    <h2 class="text-3xl w-full">Recent Projects</h2>
  </div>

  <div class="col-start-1 my-5">
    {#if recentProjects.length > 0}
      <ul class="list-none p-0 m-0 w-full overflow-scroll h-100">
        {#each recentProjects as project}
          <li
            class="p-5 mb-5 border border-gray-400 rounded-md transition duration-300 ease-in-out cursor-pointer hover:shadow-md"
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
