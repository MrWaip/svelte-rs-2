<svelte:options runes={false} />
<script>
	export let rows;
	export let key;
</script>

{#each rows as { [key]: value }}
	<input bind:value={value} />
{/each}
