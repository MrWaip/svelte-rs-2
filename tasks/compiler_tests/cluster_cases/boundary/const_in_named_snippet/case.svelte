<script>
	let { n } = $props();
	function compute() {
		return n + 1;
	}
</script>

<svelte:boundary>
	{@const value = compute()}
	<div>{@render row()}</div>

	{#snippet row()}
		<p>row</p>
	{/snippet}
</svelte:boundary>
