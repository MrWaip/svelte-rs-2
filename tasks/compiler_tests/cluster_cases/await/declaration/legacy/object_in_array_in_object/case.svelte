<svelte:options runes={false} />
<script>
	let p = Promise.resolve({ outer: [{ inner: 1 }] });
</script>
{#await p then { outer: [{ inner }] }}
	<button>{inner}</button>
{/await}
