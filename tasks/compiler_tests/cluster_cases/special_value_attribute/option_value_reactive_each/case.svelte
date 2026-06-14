<script>
	export let foo;
	export let items;
</script>

<select bind:value={foo}>
	{#each items as item}
		<option value={item}>{item.id}</option>
	{/each}
</select>
