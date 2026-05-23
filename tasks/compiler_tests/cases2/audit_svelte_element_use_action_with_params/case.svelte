<script>
	let tag = $state("div");
	function action(node, opts) {}
	let opts = $state({ x: 1 });
</script>

<svelte:element this={tag} use:action={opts}>x</svelte:element>
