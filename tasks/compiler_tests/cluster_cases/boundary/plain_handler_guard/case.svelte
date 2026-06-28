<script>
	function handleError(e) {
		console.error(e);
	}
</script>

<svelte:boundary onerror={handleError}>x</svelte:boundary>
