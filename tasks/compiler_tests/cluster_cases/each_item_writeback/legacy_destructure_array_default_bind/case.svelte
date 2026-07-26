<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [first = 'x']}
	<input bind:value={first} />
{/each}
