<script>
	let tag = $state('div');
	let isActive = $state(true);
</script>

<svelte:element this={tag} class:active={isActive}>content</svelte:element>
