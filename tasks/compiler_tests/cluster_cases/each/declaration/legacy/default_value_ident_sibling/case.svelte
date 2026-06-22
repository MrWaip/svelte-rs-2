<svelte:options runes={false} />
<script>
	let array = [{ a: 1, c: 2 }];
</script>
{#each array as { a, b = c, c }}
	<button>{a}{b}{c}</button>
{/each}
