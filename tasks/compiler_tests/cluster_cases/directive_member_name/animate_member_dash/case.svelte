<script>
	const fn = (node, options) => ({});
	let a = { b: { 'c-d': fn } };
	let directive = $derived(a);
</script>

{#each [] as i (i)}
	<div animate:directive.b.c-d></div>
{/each}
