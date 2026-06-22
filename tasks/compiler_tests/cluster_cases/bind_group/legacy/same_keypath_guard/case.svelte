<svelte:options runes={false} />
<script>
	let data = { a: 1 };
</script>

<input type="radio" bind:group={data.a} value={1} />
<input type="radio" bind:group={data.a} value={2} />
