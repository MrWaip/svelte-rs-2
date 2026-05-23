<script>
	let tag = $state("div");
	let active = $state(false);
	let primary = $state(true);
</script>

<svelte:element this={tag} class:active class:primary>x</svelte:element>
