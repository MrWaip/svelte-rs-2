<script>
	let response = await fetch('/api');
</script>

{#snippet content(value, extra)}
	<p>{value}{extra}</p>
{/snippet}

{@render content(response, await response.text())}
