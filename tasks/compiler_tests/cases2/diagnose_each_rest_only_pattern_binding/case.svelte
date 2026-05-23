<svelte:options runes={false} />
<script>
  export let items = [];
</script>

{#each items as { ...item }}
  <span>{item.x}</span>
{/each}
