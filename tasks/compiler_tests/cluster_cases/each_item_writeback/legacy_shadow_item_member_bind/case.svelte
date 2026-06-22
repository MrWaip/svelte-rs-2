<svelte:options runes={false} />
<script>
	let rows = [{ v: 1 }];
</script>

{#each rows as rows}
	<input bind:value={rows.v} />
{/each}
