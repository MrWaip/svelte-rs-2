<script>
	function handleError(error) {
		console.error(error);
	}
</script>

<svelte:boundary onerror={handleError}>
	<p>content</p>
</svelte:boundary>
