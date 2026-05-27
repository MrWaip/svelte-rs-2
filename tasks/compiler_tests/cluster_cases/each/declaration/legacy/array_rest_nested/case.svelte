<svelte:options runes={false} />
<script>
	let items = [[1, 2, 3]];
</script>
{#each items as [a, ...[b, c]]}
	<button>{a}{b}{c}</button>
{/each}
