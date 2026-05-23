<script>
	let tag = $state("div");
	let title = $state("hello");
	let color = $state("red");
</script>

<svelte:element this={tag} title={title} style:color={color}>x</svelte:element>
