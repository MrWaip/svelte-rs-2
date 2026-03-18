<script>
	let url = $state('/api');
	let promise = $derived(fetch(url));
</script>

{#await promise then value}
	<p>{value}</p>
{/await}
