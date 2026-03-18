<svelte:boundary>
	<p>content</p>

	{#snippet failed(error)}
		<p>{error.message}</p>
	{/snippet}
</svelte:boundary>
