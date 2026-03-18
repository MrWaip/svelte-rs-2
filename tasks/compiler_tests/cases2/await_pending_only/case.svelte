<script>
	const promise = fetch('/api');
</script>

{#await promise}
	<p>Loading...</p>
{/await}
