<script>
	let tag = $state("div");
	let a = $state({ id: "a" });
	let b = $state({ id: "b" });
</script>

<svelte:element this={tag} {...a} {...b}>x</svelte:element>
