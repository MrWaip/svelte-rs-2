<svelte:options runes={false} />
<script>
	let items = [[1, 2]];
</script>
{#each items as [a, b]}
	<button>{a}{b}</button>
{/each}
