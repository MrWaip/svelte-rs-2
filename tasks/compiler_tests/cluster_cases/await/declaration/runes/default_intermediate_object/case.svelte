<script>
	let p = $state(Promise.resolve({}));
</script>
{#await p then { p: { a } = {} }}
	<button>{a}</button>
{/await}
