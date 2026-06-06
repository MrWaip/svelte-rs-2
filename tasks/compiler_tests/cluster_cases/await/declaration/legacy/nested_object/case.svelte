<svelte:options runes={false} />
<script>
	let p = Promise.resolve({ p: { a: 1 }, q: { b: 2 } });
</script>
{#await p then { p: { a }, q: { b } }}
	<button>{a}{b}</button>
{/await}
