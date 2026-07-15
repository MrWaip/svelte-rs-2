<svelte:boundary>
	<p>a</p>

	{#snippet pending()}pending{/snippet}
	{#snippet failed(_, reset)}
		<button onclick={reset}>reset</button>
	{/snippet}
</svelte:boundary>
