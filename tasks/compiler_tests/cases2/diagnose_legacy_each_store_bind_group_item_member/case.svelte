<svelte:options runes={false} />
<script>
	import Row from "./Row.svelte";
	export let store;
</script>

{#each $store as item (item.id)}
	{#if item.tag}
		<Row bind:group={item.tag} />
	{/if}
{/each}
