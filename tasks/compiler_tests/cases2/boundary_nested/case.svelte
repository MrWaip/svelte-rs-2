<svelte:boundary>
	<svelte:boundary>
		<p>inner</p>

		{#snippet failed(error)}
			<p>{error.message}</p>
		{/snippet}
	</svelte:boundary>

	{#snippet failed(error)}
		<p>outer: {error.message}</p>
	{/snippet}
</svelte:boundary>
