{#snippet row(item)}
	{const label = item.name}
	<p>{label}</p>
{/snippet}
{@render row({ name: "x" })}
