<svelte:options runes={false} />
<script>
	export let items;
	const { list } = items;
</script>

{#each $list as { value }}
	<Child bind:prop={value} />
{/each}
