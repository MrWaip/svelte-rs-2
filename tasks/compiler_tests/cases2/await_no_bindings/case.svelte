<script>
	const promise = fetch('/api');
</script>

{#await promise}
	<p>Loading</p>
{:then}
	<p>Done</p>
{:catch}
	<p>Error</p>
{/await}
