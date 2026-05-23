<svelte:options runes={false} />
<script>
  export let items = [];
</script>

{#each items as [a, ...rest]}
  <span>{a}{rest[0]}</span>
{/each}
