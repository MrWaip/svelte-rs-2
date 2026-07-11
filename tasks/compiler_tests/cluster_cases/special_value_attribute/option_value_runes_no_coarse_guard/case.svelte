<script>
	let { items } = $props();
</script>

<select>
	{#each items as item}
		<option value={item.value}>{item.text}</option>
	{/each}
</select>
