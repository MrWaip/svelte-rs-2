<svelte:options runes={false} />
<script>
	let p = Promise.resolve([1, 2]);
</script>
{#await p then [a, b]}
	<button>{a}{b}</button>
{/await}
