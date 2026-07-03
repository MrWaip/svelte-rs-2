<script>
	let { n } = $props();
	function compute() {
		return n + 1;
	}
</script>

<svelte:boundary>
	{@const value = compute()}
	<div>{value}</div>

	{#snippet failed()}
		<p>failed</p>
	{/snippet}
</svelte:boundary>
