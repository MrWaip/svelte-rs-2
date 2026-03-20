<script>
	let items = $state([1, 2, 3]);
</script>

<svelte:boundary>
	{@const x = items.length}
	<p>{x}</p>
	{#snippet failed(error)}
		<p>{x}: {error.message}</p>
	{/snippet}
</svelte:boundary>
