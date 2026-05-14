<script>
	let tag = $state("div");
	let active = $state(false);
</script>

<svelte:element this={tag} class:active>x</svelte:element>

<style>
	.active { color: red; }
</style>
