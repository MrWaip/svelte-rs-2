<svelte:options runes={false} />
<script>
	let list = [{ rows: [{ data: [] }] }];
</script>

{#each list as group}
	{#each group.rows as item}
		<input type="checkbox" bind:group={item.data} value="a" />
	{/each}
{/each}
