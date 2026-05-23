<script>
	let tag = $state("div");
	let title = $state("hello");
	let active = $state(false);
</script>

<svelte:element this={tag} title={title} class:active>x</svelte:element>
