<script>
	let p = $state(Promise.resolve({ a: 1, b: 2 }));
</script>
{#await p then { a: x, b: y }}
	<button>{x}{y}</button>
{/await}
