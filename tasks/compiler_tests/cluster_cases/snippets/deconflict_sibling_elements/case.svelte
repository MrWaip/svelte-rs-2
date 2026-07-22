<div>
	{#snippet row(n)}<span>{n}</span>{/snippet}
	{@render row(1)}
</div>
<div>
	{#snippet row(n)}<b>{n}</b>{/snippet}
	{@render row(2)}
</div>
