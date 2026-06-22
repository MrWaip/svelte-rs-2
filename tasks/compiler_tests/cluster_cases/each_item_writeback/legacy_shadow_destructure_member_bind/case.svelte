<svelte:options runes={false} />
<script>
	let a = [{ a: { b: 'x' }, key: 'b' }];
</script>

{#each a as { a, key }}
	<input bind:value={a[key]} />
{/each}
