<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [entry]}
	<button on:click={() => (entry.count = 2)}>{entry.count}</button>
{/each}
