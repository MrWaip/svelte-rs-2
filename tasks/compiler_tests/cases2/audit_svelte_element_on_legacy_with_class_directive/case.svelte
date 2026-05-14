<script>
	let tag = $state("div");
	let active = $state(false);
	function handler() {}
</script>

<svelte:element this={tag} on:click={handler} class:active>x</svelte:element>
