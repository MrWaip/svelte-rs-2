<script>
	let tag = $state('div');
	let value = $state('red');
	const getClass = () => value === 'blue';
	const getValue = () => value;
</script>

<svelte:element this={tag} class:blue={getClass()} style:color={getValue()}></svelte:element>
<button onclick={() => value = 'blue'}>toggle</button>
