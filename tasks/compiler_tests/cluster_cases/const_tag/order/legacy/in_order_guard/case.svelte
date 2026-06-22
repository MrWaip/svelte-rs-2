<svelte:options runes={false} />
{#if true}
	{@const bar = 'world'}
	{@const foo = bar}
	<p>{foo}</p>
{/if}
