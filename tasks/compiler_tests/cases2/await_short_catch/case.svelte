<script>
	const promise = fetch('/api');
</script>

{#await promise catch error}
	<p>{error.message}</p>
{/await}
