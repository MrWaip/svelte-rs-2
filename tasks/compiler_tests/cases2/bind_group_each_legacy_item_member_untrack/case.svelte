<svelte:options runes={false} />
<script>
	let items = [{ id: 'a' }, { id: 'b' }];
	let selected;
</script>

{#each items as item}
	<input type="checkbox" value={item.id} bind:group={selected} />
{/each}
