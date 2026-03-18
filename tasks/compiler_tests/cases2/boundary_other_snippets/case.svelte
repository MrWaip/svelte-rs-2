<svelte:boundary>
	<p>content</p>

	{#snippet helper()}
		<span>helper text</span>
	{/snippet}

	{#snippet failed(error)}
		<p>{error.message}</p>
		{@render helper()}
	{/snippet}
</svelte:boundary>
