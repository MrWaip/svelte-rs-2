<script>
	let v = $state({ a: 1, b: 2 });
</script>
{#snippet s({ a: x, b: y })}
	<button>{x}{y}</button>
{/snippet}
{@render s(v)}
