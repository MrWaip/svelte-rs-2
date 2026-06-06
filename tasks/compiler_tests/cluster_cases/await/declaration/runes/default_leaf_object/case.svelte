<script>
	let p = $state(Promise.resolve({}));
</script>
{#await p then { a = 10, b = 20 }}
	<button>{a}{b}</button>
{/await}
