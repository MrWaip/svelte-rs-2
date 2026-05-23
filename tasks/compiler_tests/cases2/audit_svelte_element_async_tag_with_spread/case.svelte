<script>
	let props = $state({ id: "x" });
	async function getTag() { return "div"; }
</script>

<svelte:element this={await getTag()} {...props}>x</svelte:element>
