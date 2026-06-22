<svelte:options runes={false} />
<script>
	export let selected_array;
	export let values;
</script>

{#each selected_array as selected}
	{#each values as value}
		<input type="checkbox" value="{value}" bind:group={selected} />
	{/each}
{/each}
