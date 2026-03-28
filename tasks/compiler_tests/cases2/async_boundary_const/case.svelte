<script>
	let x = $state(1);
</script>

<svelte:boundary>
	{@const doubled = x * 2}
	<p>{doubled}</p>

	{#snippet failed(error)}
		<p>{error.message}</p>
	{/snippet}
</svelte:boundary>
