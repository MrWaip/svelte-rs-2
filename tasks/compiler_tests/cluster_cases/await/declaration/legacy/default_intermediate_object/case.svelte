<svelte:options runes={false} />
<script>
	let p = Promise.resolve({});
</script>
{#await p then { p: { a } = {} }}
	<button>{a}</button>
{/await}
