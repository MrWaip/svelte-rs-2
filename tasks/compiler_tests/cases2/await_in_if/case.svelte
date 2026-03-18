<script>
	let show = $state(true);
	const promise = fetch('/api');
</script>

{#if show}
	{#await promise then value}
		<p>{value}</p>
	{/await}
{/if}
