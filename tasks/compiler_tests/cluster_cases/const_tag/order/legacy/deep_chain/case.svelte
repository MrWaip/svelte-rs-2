<svelte:options runes={false} />
{#if true}
	{@const a = b}
	{@const b = c}
	{@const c = 'x'}
	<p>{a}</p>
{/if}
