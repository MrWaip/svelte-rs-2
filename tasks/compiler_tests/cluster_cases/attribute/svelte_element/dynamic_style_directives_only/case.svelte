<script>
	let tag = $state('div');
	let value = $state('red');
	const getValue = () => value;
</script>

<svelte:element this={tag} style:color={getValue()}></svelte:element>
<button onclick={() => value = 'blue'}>toggle</button>
