<svelte:options runes={false} />
<script>
	export let store;
</script>

{#each $store as item (item.id)}
	{#if item.value}
		<input bind:value={item.value} />
	{/if}
{/each}
