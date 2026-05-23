<script>
	let tag = $state("div");
	let a = $state("a");
	let b = $state("b");
	let active = $state(false);
</script>

<svelte:element this={tag} class={[a, b]} class:active>x</svelte:element>
