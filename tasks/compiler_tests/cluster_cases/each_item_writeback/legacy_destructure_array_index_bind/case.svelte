<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [first, second]}
	<input bind:value={first} />
	<span>{second}</span>
{/each}
