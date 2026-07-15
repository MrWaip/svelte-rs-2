<script>
	let { x } = $props();
</script>

<svelte:boundary>
	<p>a</p>

	{#snippet pending()}{x} loading{/snippet}
	{#snippet failed(_, reset)}
		<button onclick={reset}>reset</button>
	{/snippet}
</svelte:boundary>
