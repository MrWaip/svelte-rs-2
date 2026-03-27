<script>
	let { tag = "p", name } = $props();
</script>

<svelte:element this={tag}>Hello {name}!</svelte:element>
