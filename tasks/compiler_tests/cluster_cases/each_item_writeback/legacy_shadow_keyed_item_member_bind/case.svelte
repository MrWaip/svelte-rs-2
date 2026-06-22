<svelte:options runes={false} />
<script>
	let a = [{ id: 1, v: 'x' }];
</script>

{#each a as a (a.id)}
	<input bind:value={a.v} />
{/each}
