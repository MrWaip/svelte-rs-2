<script>
	async function getTag() {
		return 'div';
	}
</script>

<svelte:element this={await getTag()} />
