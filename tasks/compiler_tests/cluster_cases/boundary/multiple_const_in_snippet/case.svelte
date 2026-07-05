<script>
	let { n } = $props();
	function compute() {
		return n + 1;
	}
</script>

<svelte:boundary>
	{@const a = compute()}
	{@const b = compute()}
	<div>{a}{b}</div>

	{#snippet failed()}
		<p>failed</p>
	{/snippet}
</svelte:boundary>
