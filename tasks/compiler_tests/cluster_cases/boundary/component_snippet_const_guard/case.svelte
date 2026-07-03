<script>
	import Row from './Row.svelte';
	let { n } = $props();
	function compute() {
		return n + 1;
	}
</script>

<Row>
	{@const value = compute()}
	<div>{value}</div>

	{#snippet cell()}
		<p>cell</p>
	{/snippet}
</Row>
