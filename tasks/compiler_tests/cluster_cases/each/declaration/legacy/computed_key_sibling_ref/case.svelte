<svelte:options runes={false} />
<script>
	let items = [[1, 2, 3]];
</script>
{#each items as { length, [length - 1]: last, [Math.floor(length / 2)]: mid }}
	<button>{last}{mid}{length}</button>
{/each}
