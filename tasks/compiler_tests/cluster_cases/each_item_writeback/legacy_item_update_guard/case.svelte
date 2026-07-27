<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as item}
	<button on:click={() => item++}>{item}</button>
{/each}
