<svelte:options runes={false} />
<script>
	let arr = [{ prop: "foo" }];
</script>

{#each arr as o}
	<span>{o.prop}</span>
	<button on:click={() => o = { ...o, prop: "bar" }}>x</button>
{/each}
