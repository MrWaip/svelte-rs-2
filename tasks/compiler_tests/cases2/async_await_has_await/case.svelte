<script>
	async function getPromise() {
		return fetch('/api');
	}
</script>

{#await await getPromise() then value}
	<p>{value}</p>
{/await}
