<script>
	let items = $state([{ name: 'a' }]);
</script>

{#each items as item (item)}
	<button onclick={() => item.name += '!'}>{item.name}</button>
{/each}
