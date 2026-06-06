<script>
	let v = $state({ a: 1, b: 2, c: 3 });
</script>
{#snippet s({ a, ...rest })}
	<button>{a}{rest.b}</button>
{/snippet}
{@render s(v)}
