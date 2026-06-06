<script>
	let p = $state(Promise.resolve([1, 2]));
</script>
{#await p then [a, b]}
	<button>{a}{b}</button>
{/await}
