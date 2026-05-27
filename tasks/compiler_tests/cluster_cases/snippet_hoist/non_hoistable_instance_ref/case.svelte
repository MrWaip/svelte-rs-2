<script>
	let name = 'x';
</script>

{#snippet foo()}
	{name}
{/snippet}

{@render foo()}
