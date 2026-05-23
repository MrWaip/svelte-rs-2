<script>
	let tag = $state("div");
	let active = $state(false);
</script>

<svelte:element this={tag} class="my-cls" class:active>x</svelte:element>
