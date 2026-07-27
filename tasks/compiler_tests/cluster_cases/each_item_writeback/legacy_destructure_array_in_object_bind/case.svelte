<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as { name: [first] }}
	<input bind:value={first} />
{/each}
