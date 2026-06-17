<svelte:options runes={false} />
<script>
	let list = [{ id: 'x', data: [{ id: 1, data: [] }] }];
</script>

{#each list as { id, data }}
	{#each data as item}
		<input type="checkbox" bind:group={item.data} value="a" data-index="{id}-{item.id}" />
	{/each}
{/each}
