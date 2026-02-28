<script lang="ts">
  export let name: string;
</script>

<div>
  <h2>{name} Tab Content</h2>
  <p>This is the content for the {name} tab.</p>
</div>

<style>
  div {
    padding: 20px;
    border: 1px solid var(--borderColor);
    background-color: var(--secondaryColor);
    border-radius: 5px;
  }
  h2 {
    color: var(--textColor);
  }
  p {
    color: var(--textColor);
  }
</style>
