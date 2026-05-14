<script>
	let tag = $state("div");
	let color = $state("red");
</script>

<svelte:element this={tag} style:color={color}>x</svelte:element>

<style>
	.x { color: blue; }
</style>
