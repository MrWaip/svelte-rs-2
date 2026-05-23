<script>
	let tag = $state("div");
	let a = $state("a");
	let b = $state("b");
</script>

<svelte:element this={tag} class={[a, b]}>x</svelte:element>
