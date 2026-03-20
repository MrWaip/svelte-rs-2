<script>
	let items = $state([{id: 1, name: 'a'}, {id: 2, name: 'b'}]);
	let selected = $state([]);
</script>

{#each items as item (item.id)}
	<input type="checkbox" bind:group={selected} value={item.name}>
{/each}
