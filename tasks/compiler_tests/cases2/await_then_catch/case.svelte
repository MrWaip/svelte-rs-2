<script>
	const promise = fetch('/api');
</script>

{#await promise then value}
	<p>{value}</p>
{:catch error}
	<p>{error.message}</p>
{/await}
