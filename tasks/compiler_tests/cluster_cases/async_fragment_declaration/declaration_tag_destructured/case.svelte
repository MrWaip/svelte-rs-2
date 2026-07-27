<script>
	let source = $state({ x: 1, y: 2 });
</script>

{const { x, y } = await Promise.resolve(source)}
<p>{x}{y}</p>
<button onclick={() => (source = { x: 3, y: 4 })}>go</button>
