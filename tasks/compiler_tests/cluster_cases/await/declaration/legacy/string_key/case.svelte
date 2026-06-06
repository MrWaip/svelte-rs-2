<svelte:options runes={false} />
<script>
	let p = Promise.resolve({ "a-b": 1, "c d": 2 });
</script>
{#await p then { "a-b": ab, "c d": cd }}
	<button>{ab}{cd}</button>
{/await}
