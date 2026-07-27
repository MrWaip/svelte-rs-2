<script>
	let a = await Promise.resolve([1]);
</script>
<svelte:boundary>
	{@const b = await Promise.resolve([2])}
	{#each [...b, ...a] as x}<p>{x}</p>{/each}
</svelte:boundary>
