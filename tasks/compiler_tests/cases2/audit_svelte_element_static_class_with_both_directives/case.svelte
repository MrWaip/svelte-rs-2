<script>
	let tag = $state("div");
	let active = $state(false);
	let color = $state("red");
</script>

<svelte:element this={tag} class="foo" class:active style:color={color}>x</svelte:element>
