<script>
	let tag = $state("div");
</script>

<svelte:element this={tag} class="dynamic">content</svelte:element>

<style>
	.dynamic { color: blue; }
</style>
