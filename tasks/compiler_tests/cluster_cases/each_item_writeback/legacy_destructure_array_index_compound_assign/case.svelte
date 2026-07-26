<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [first]}
	<button on:click={() => (first += 2)}>{first}</button>
{/each}
