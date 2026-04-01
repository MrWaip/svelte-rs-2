<script>
	const outer = fetch('/api/list');
</script>

{#await outer then items}
	{#await items[0] then detail}
		<p>{detail}</p>
	{/await}
{/await}
