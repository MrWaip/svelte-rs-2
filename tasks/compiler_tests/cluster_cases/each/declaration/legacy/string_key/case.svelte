<svelte:options runes={false} />
<script>
	let items = [{ "a-b": 1, "c d": 2 }];
</script>
{#each items as { "a-b": ab, "c d": cd }}
	<button>{ab}{cd}</button>
{/each}
