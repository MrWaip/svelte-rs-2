<script>
	let tag = $state("div");
	let active = $state(false);
</script>

<svelte:element this={tag} class="foo" class:active>x</svelte:element>
