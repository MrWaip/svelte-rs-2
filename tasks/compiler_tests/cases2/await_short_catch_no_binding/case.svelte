<script>
	const promise = fetch('/api');
</script>

{#await promise catch}
	<p>Something went wrong</p>
{/await}
