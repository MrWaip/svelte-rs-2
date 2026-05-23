<script>
	let tag = $state("div");
	let props = $state({ id: "x" });
</script>

<svelte:element this={tag} {...props}>x</svelte:element>

<style>
	.x { color: red; }
</style>
