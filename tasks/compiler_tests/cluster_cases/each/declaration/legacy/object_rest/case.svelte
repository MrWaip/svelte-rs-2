<svelte:options runes={false} />
<script>
	let items = [{ a: 1, b: 2, c: 3 }];
</script>
{#each items as { a, ...rest }}
	<button>{a}{rest.b}</button>
{/each}
