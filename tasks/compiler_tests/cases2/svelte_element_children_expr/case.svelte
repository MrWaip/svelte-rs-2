<script>
	let tag = $state("p");
	let name = $state("world");
</script>

<svelte:element this={tag}>Hello {name}!</svelte:element>
