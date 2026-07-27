<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as { count }}
	<button on:click={() => (count = 5)}>{count}</button>
{/each}
