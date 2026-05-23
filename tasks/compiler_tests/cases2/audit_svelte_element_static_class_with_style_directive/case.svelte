<script>
	let tag = $state("div");
	let color = $state("red");
</script>

<svelte:element this={tag} class="foo" style:color={color}>x</svelte:element>
