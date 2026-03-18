<script>
	function handleError(error) {
		console.error(error);
	}
</script>

<svelte:boundary onerror={handleError}>
	<p>content</p>

	{#snippet pending()}
		<p>loading...</p>
	{/snippet}

	{#snippet failed(error)}
		<p>{error.message}</p>
	{/snippet}
</svelte:boundary>
