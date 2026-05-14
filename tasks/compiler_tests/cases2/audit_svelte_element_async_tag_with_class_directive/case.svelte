<script>
	let active = $state(false);
	async function getTag() { return "div"; }
</script>

<svelte:element this={await getTag()} class:active>x</svelte:element>
