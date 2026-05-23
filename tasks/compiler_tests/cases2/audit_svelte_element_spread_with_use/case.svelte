<script>
	let tag = $state("div");
	let props = $state({ id: "x" });
	function action(node) {}
</script>

<svelte:element this={tag} {...props} use:action>x</svelte:element>
