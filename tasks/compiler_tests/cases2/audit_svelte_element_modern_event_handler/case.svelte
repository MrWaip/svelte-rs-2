<script>
	let tag = $state("div");
	let count = $state(0);
</script>

<svelte:element this={tag} onclick={() => count++}>x</svelte:element>
