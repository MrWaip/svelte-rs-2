<svelte:options runes={false} />
<script>
	export let selected_array;
	export let selected_index;
	export let options;
</script>

{#each selected_array as selected}
	{#each selected_index as index}
		{#each options as value}
			<input type="checkbox" bind:group={selected[index]} value={value} />
		{/each}
	{/each}
{/each}
