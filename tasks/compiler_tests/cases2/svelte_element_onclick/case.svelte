<script>
	let tag = $state("button");
	let count = $state(0);
</script>

<svelte:element this={tag} onclick={() => count++}>Click</svelte:element>
