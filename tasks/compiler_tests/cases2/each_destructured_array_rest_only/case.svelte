<svelte:options runes={false} />
<script>
  export let items = [];
</script>

{#each items as [...rest]}
  <span>{rest[0]}</span>
{/each}
