<script>
	let p = $state(Promise.resolve({ p: { a: 1 }, q: { b: 2 } }));
</script>
{#await p then { p: { a }, q: { b } }}
	<button>{a}{b}</button>
{/await}
