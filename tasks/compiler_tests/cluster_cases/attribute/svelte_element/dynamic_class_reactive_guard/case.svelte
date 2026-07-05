<script>
	let tag = $state('div');
	let flag = $state(false);
</script>

<svelte:element this={tag} class:blue={flag} style:color={flag ? 'red' : 'blue'}></svelte:element>
<button onclick={() => flag = !flag}>toggle</button>
