<script>
	let categories = $state([
		{ name: 'fruit', selected: [] },
		{ name: 'veg', selected: [] }
	]);
</script>

{#each categories as category}
	<input type="checkbox" bind:group={category.selected} value="apple">
{/each}
