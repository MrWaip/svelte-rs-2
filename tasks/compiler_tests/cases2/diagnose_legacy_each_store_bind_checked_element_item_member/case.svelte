<svelte:options runes={false} />
<script>
	export let store;
</script>

{#each $store as item (item.id)}
	{#if item.enabled}
		<input type="checkbox" bind:checked={item.enabled} />
	{/if}
{/each}
