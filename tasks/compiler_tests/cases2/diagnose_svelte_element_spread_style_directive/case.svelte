<script>
	let tag = $state("div");
	let props = $state({ id: "bar" });
	let color = $state("red");
</script>

<svelte:element this={tag} {...props} style:color={color}>x</svelte:element>
