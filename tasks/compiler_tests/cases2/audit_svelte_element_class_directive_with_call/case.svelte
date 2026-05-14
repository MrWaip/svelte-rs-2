<script>
	let tag = $state("div");
	let n = $state(0);
	function compute(x) { return x > 0; }
</script>

<svelte:element this={tag} class:active={compute(n)}>x</svelte:element>
