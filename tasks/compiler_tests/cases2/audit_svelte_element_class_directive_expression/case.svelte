<script>
	let tag = $state("div");
	let n = $state(0);
</script>

<svelte:element this={tag} class:active={n > 0}>x</svelte:element>
