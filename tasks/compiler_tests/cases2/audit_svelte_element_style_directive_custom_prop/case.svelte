<script>
	let tag = $state("div");
	let value = $state("red");
</script>

<svelte:element this={tag} style:--accent={value}>x</svelte:element>
