<script>
	let { tag } = $props();
	async function g() { return 2 }
</script>
<svelte:element this={tag} title={await g()} />
