<script>
	const fn = (node, options) => ({});
	let a = { b: { 'c-d': fn } };
	let directive = $derived(a);
</script>

<div in:directive.b.c-d></div>
