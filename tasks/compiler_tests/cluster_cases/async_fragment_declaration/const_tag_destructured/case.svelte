<script>
	let source = $state({ x: 1, y: 2 });
</script>

{#if source}
	{@const { x, y } = await Promise.resolve(source)}
	<p>{x}{y}</p>
{/if}
<button onclick={() => (source = { x: 3, y: 4 })}>go</button>
