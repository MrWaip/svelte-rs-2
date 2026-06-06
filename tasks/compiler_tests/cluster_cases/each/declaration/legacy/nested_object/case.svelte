<svelte:options runes={false} />
<script>
	let items = [{ p: { a: 1 }, q: { b: 2 } }];
</script>
{#each items as { p: { a }, q: { b } }}
	<button>{a}{b}</button>
{/each}
