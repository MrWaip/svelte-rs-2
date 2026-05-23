<script>
	let tag = $state("div");
	let obj = $state({ el: null });
</script>

<svelte:element this={tag} bind:this={obj.el}>x</svelte:element>
