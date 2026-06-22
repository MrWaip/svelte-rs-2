<svelte:options runes={false} />
<script>
	let arr = [1, 2, 3];
</script>

{#each arr as o}
	<button on:click={() => { o *= 2; }}>{o}</button>
{/each}
