<script>
	function failed($$anchor, error) {
		console.log('attribute', error);
	}
</script>

<svelte:boundary {failed}>
	<p>content</p>

	{#snippet failed(error)}
		<p>{error.message}</p>
	{/snippet}
</svelte:boundary>
