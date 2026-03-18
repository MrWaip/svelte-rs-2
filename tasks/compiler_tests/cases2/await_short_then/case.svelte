<script>
	const promise = fetch('/api');
</script>

{#await promise then value}
	<p>{value}</p>
{/await}
