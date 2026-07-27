<svelte:boundary>
	{@const number = await Promise.resolve(5)}
	{#snippet greet()}
		<h1>{number}</h1>
	{/snippet}
	{@render greet()}
</svelte:boundary>
