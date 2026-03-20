<script>
	let tag = $state('div');
	let col = $state('red');
</script>

<svelte:element this={tag} style:color={col}>content</svelte:element>
