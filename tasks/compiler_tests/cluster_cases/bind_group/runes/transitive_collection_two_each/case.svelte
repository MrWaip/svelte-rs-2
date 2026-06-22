<script>
	let list = $state([{ data: [{ value: [] }] }]);
</script>

{#each list as { data }}
	{#each data as item}
		<input type="checkbox" bind:group={item.value} value="a" />
	{/each}
{/each}
