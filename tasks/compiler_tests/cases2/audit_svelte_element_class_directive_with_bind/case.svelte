<script>
	let tag = $state("div");
	let active = $state(false);
	let el = $state();
</script>

<svelte:element this={tag} class:active bind:this={el}>x</svelte:element>
