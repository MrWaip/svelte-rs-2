<svelte:options runes={false} />
<script>
	let p = Promise.resolve([1]);
</script>
{#await p then [a = 10, b = 20]}
	<button>{a}{b}</button>
{/await}
