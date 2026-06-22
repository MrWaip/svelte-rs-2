<svelte:options runes={false} />
<script>
	let items = [{}];
</script>
{#each items as { width = 10, area = Math.max(width, 0) }}
	<button>{width}{area}</button>
{/each}
