<svelte:options runes={false} />
<script>
	let one = 1;
	let two = 2;
</script>

<input type="radio" bind:group={one} value={1} />
<input type="radio" bind:group={two} value={2} />
