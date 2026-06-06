<script>
	let p = $state(Promise.resolve({ outer: [{ inner: 1 }] }));
</script>
{#await p then { outer: [{ inner }] }}
	<button>{inner}</button>
{/await}
