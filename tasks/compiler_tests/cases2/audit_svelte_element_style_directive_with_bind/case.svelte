<script>
	let tag = $state("div");
	let color = $state("red");
	let el = $state();
</script>

<svelte:element this={tag} style:color={color} bind:this={el}>x</svelte:element>
