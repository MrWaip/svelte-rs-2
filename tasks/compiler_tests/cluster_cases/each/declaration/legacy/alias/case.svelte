<svelte:options runes={false} />
<script>
	let items = [{ a: 1, b: 2 }];
</script>
{#each items as { a: x, b: y }}
	<button>{x}{y}</button>
{/each}
