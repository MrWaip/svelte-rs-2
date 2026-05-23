<svelte:options runes={false} />
<script>
	let items = ['a', 'b'];
	let selected;
</script>

{#each items as item}
	<input type="checkbox" value={item} bind:group={selected} />
{/each}
