<svelte:options runes={false} />
{#if true}
	{@const a = b}
	{@const b = a}
	<p>{a}{b}</p>
{/if}
