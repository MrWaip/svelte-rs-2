<script>
	let tag = $state("div");
	let items = $state([{ id: 1, active: false }, { id: 2, active: true }]);
</script>

{#each items as item (item.id)}
	<svelte:element this={tag} class:active={item.active}>{item.id}</svelte:element>
{/each}
