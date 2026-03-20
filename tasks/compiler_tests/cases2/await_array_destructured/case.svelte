<script>
	const promise = fetch('/api');
</script>

{#await promise then [a, b]}
	<p>{a} and {b}</p>
{/await}
