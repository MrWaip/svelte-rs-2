<script>
	let { n } = $props();
</script>

<svelte:boundary>
	<div>{n}</div>

	{#snippet failed()}
		<p>failed</p>
	{/snippet}
</svelte:boundary>
