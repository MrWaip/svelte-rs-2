<script>
	let tag = $state("div");
	let color = $state("red");
	let bg = $state("white");
</script>

<svelte:element this={tag} style:color={color} style:background={bg}>x</svelte:element>
