<script>
	export let store;
	const { items } = store;
</script>

{#each $items.list as item}
	<div>{item}</div>
{/each}
