<script>
	let source = $state({ x: 1, y: 2 });
</script>

{#if source}
	{@const a = source.x}
	{@const { x, y } = source}
	<p>{a}{x}{y}</p>
{/if}
<button onclick={() => (source = { x: 3, y: 4 })}>go</button>
