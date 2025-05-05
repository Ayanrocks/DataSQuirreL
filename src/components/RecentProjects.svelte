<script lang="ts">
  // Placeholder data for recent projects
  const recentProjects = [
    { id: 1, name: 'Project Alpha' },
    { id: 2, name: 'Project Beta' },
    { id: 3, name: 'Project Gamma' },
  ];
</script>

<style>
  /* No component-specific styles needed with Tailwind */
</style>

<h2 class="text-lg font-semibold mb-4">Recent Projects</h2>

{#if recentProjects.length > 0}
  <ul class="list-none p-0 m-0">
    {#each recentProjects as project}
      <li class="p-2 mb-1 border border-gray-200 rounded-md transition duration-300 ease-in-out cursor-pointer hover:shadow-md">
        {project.name}
      </li>
    {/each}
  </ul>
{:else}
  <div class="flex items-center justify-center h-32 text-gray-500">
    No recent Project
  </div>
{/if}
