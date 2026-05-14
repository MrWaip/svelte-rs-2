<script>
	let tag = $state("div");
	let props = $state({ id: "bar" });
	let active = $state(false);
</script>

<svelte:element this={tag} {...props} class:active>x</svelte:element>
