<script>
	const promise = fetch('/api');
</script>

{#await promise}
	<p>Loading...</p>
{:catch error}
	<p>{error.message}</p>
{/await}
