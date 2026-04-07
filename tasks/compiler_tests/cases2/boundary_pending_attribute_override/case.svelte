<script>
	function pending($$anchor) {
		console.log('attribute', $$anchor);
	}
</script>

<svelte:boundary {pending}>
	<p>content</p>

	{#snippet pending()}
		<p>loading...</p>
	{/snippet}
</svelte:boundary>
