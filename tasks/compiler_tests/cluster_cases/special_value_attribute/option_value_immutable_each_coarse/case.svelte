<svelte:options immutable={true} />
<script>
	export let items;
</script>

<select>
	{#each items as item}
		<option value={item.value}>{item.text}</option>
	{/each}
</select>
