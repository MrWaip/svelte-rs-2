<svelte:options runes={false} />
{#if true}
	{@const a = 'x'}
	{@const b = 'y'}
	<p>{a}{b}</p>
{/if}
