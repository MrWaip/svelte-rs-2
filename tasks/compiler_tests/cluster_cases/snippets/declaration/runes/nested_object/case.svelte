<script>
	let v = $state({ p: { a: 1 }, q: { b: 2 } });
</script>
{#snippet s({ p: { a }, q: { b } })}
	<button>{a}{b}</button>
{/snippet}
{@render s(v)}
