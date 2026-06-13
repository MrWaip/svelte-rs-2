<svelte:options runes={false} />
<script>
	export let items;
	const { list } = items;
</script>

{#each $list as { value }, idx}
	<Child bind:prop={value} />
{/each}
