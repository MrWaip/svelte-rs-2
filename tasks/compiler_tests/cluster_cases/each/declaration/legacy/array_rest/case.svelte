<svelte:options runes={false} />
<script>
	let items = [[1, 2, 3]];
</script>
{#each items as [a, ...rest]}
	<button>{a}{rest.length}</button>
{/each}
