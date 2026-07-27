<svelte:boundary>
	{@const number = await Promise.resolve(5)}
	{#snippet greet()}
		{@const greeting = await 'hi'}
		{#if number > 4 && greeting}
			<p>yes</p>
		{/if}
	{/snippet}
	{@render greet()}
</svelte:boundary>
