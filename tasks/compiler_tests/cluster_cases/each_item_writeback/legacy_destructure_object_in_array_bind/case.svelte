<svelte:options runes={false} />
<script>
	export let rows;
</script>

{#each rows as [{ name }]}
	<input bind:value={name} />
{/each}
