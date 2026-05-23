<script>
	let tag = $state("div");
	let color = $state("red");
</script>

<svelte:element this={tag} style:color|important={color}>x</svelte:element>
