<script>
	function handleError(error) {
		console.error(error);
	}
</script>

<svelte:boundary onerror={handleError}>
	<p>content</p>

	{#snippet failed(error)}
		<p>{error.message}</p>
	{/snippet}
</svelte:boundary>
