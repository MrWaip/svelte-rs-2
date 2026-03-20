<script>
	let groups = $state([['a', 'b'], ['c', 'd']]);
	let selected = $state([]);
</script>

{#each groups as group}
	{#each group as item}
		<input type="checkbox" bind:group={selected} value={item}>
	{/each}
{/each}
