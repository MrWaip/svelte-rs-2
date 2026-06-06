<script>
	let p = $state(Promise.resolve({ a: 1, b: 2, c: 3 }));
</script>
{#await p then { a, ...rest }}
	<button>{a}{rest.b}</button>
{/await}
