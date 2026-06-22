<svelte:options runes={false} />
<script>
	let items = [{ id: 'a' }];
</script>

{#each items as item, idx (item.id)}
	<input bind:value={item} />
{/each}
