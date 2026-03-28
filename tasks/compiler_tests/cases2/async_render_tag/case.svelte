<script>
	let data = await fetch('/api');
</script>

{#snippet content(value)}
	<p>{value}</p>
{/snippet}

{@render content(data)}
