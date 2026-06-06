<svelte:options runes={false} />
<script>
	let items = [[1, 2, 3]];
</script>
{#each items as [a, , c]}
	<button>{a}{c}</button>
{/each}
