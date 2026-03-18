<script>
	const promise = fetch('/api');
</script>

{#await promise}
	<div class="loading">
		<span>Please wait...</span>
	</div>
{:then value}
	<div class="result">
		<h1>Result</h1>
		<p>{value}</p>
	</div>
{:catch error}
	<div class="error">
		<h1>Error</h1>
		<p>{error.message}</p>
	</div>
{/await}
