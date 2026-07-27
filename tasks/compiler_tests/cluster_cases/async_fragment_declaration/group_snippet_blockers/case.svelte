{#if true}
	{@const number = await Promise.resolve(5)}
	{#snippet greet()}
		<h1>{number}</h1>
	{/snippet}
	{@render greet()}
{/if}
