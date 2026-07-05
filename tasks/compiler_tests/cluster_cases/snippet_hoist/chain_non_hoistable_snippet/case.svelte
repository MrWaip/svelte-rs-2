<script>
	let abc = 'a';
</script>

{@render c()}

{#snippet a()}
	{abc}
{/snippet}

{#snippet b()}
	{@render a()}
{/snippet}

{#snippet c()}
	{@render b()}
{/snippet}
