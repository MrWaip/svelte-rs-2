<svelte:options runes={false} />
<script>
	export let people;
</script>

{#each people as { name: { first: f, last: l } }}
	<input bind:value={f} />
	<input bind:value={l} />
{/each}
