<script>
	let tag = $state("div");
	let props = $state({ id: "bar" });
	let active = $state(false);
	let el = $state();
</script>

<svelte:element this={tag} {...props} class:active bind:this={el}>x</svelte:element>
