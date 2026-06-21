<svelte:options runes={false} />
<script>
	let data = { a: 1, b: [] };
	let items = ['x', 'y'];
</script>

<input type="radio" bind:group={data.a} value={1} />
<input type="radio" bind:group={data.a} value={2} />
{#each items as item}
	<input type="checkbox" bind:group={data.b} value={item} />
{/each}
