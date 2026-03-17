<script>
	let tag = $state("div");
	let props = $state({ class: "foo", id: "bar" });
</script>

<svelte:element this={tag} {...props}>content</svelte:element>
