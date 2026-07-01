<script>
	let tag = $state('div');
	let value = $state('red');
	const getClass = () => value === 'blue';
</script>

<svelte:element this={tag} class:blue={getClass()}></svelte:element>
<button onclick={() => value = 'blue'}>toggle</button>
