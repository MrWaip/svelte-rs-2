<svelte:options runes={false} />
<script>
	let list = ['Hello'];
</script>

{#each list as item}
	<input bind:value={item} />
{/each}
