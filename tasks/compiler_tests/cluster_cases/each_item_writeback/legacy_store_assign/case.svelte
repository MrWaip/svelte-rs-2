<svelte:options runes={false} />
<script>
	export let items;
	const { list } = items;
</script>

{#each $list as item, idx}
	<button on:click={() => item = item + 1}>{item}</button>
{/each}
