<script>
	let promise = $state(Promise.resolve(42));
</script>

{#await promise then value}
	{@const doubled = value * 2}
	<p>{doubled}</p>
{/await}
