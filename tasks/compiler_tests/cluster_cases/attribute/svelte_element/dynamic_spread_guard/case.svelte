<script>
	let tag = $state('div');
	let value = $state('red');
	const getSpread = () => ({ class: value });
</script>

<svelte:element this={tag} {...getSpread()}></svelte:element>
<button onclick={() => value = 'blue'}>toggle</button>
