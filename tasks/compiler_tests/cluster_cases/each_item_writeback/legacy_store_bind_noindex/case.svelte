<svelte:options runes={false} />
<script>
	export let items;
	const { list } = items;
</script>

{#each $list as item}
	<Child bind:value={item} />
{/each}
