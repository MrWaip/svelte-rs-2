<script>
	let { header, headerTag = 'div' } = $props();
	let cond = $state(true);
</script>

<svelte:element this={headerTag}>
	{@render header()}
	{#if cond}
		<span>x</span>
	{/if}
</svelte:element>
