<script>
	let items = $state([{ id: 1, name: 'a' }]);
</script>

{#each items as item (item.id)}
	<button onclick={() => item.name += '!'}>{item.name}</button>
{/each}
