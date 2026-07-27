<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [first, second, ...[third, ...{ length }]]}
	<span>{first}{second}</span>
	<input bind:value={third} />
	<input bind:value={length} />
{/each}
