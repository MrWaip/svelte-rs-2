<svelte:options runes={false} />
<script>
	let p = Promise.resolve([1, 2, 3]);
</script>
{#await p then [a, , c]}
	<button>{a}{c}</button>
{/await}
