<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [first]}
	<input type="checkbox" bind:group={first} value="a" />
{/each}
