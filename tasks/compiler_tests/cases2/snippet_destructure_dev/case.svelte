{#snippet greeting({ label, name = "world" })}
	<p>{label}: {name}</p>
{/snippet}

{@render greeting({ label: "Hi" })}
