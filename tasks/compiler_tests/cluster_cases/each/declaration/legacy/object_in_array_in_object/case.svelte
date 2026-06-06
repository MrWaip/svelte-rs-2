<svelte:options runes={false} />
<script>
	let items = [{ outer: [{ inner: 1 }] }];
</script>
{#each items as { outer: [{ inner }] }}
	<button>{inner}</button>
{/each}
