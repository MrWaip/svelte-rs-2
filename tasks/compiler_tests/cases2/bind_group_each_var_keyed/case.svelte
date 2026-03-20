<script>
	let categories = $state([
		{ id: 1, name: 'fruit', selected: [] },
		{ id: 2, name: 'veg', selected: [] }
	]);
</script>

{#each categories as category (category.id)}
	<input type="checkbox" bind:group={category.selected} value="apple">
{/each}
