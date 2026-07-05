<script>
	let items = $state([{ name: 'a' }]);
</script>

{#each items as item (item)}
	<p>{item.name}</p>
{/each}
