<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [first]}
	<button on:click={() => (first = 'x')}>{first}</button>
{/each}
