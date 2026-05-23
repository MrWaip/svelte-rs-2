<script>
	let tag = $state("div");
	let active = $state(false);
	function action(node) {}
</script>

<svelte:element this={tag} class:active use:action>x</svelte:element>
