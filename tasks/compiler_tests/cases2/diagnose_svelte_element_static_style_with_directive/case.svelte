<script>
	let tag = $state("div");
	let color = $state("red");
</script>

<svelte:element this={tag} style="font-size: 12px" style:color={color}>x</svelte:element>
