<script>
	let tag = $state("div");
	let title = $state("hi");
	let active = $state(false);
	let color = $state("red");
</script>

<svelte:element this={tag} title={title} class:active style:color={color}>x</svelte:element>
