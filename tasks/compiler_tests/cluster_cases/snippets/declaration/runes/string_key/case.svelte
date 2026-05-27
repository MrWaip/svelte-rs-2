<script>
	let v = $state({ "a-b": 1, "c d": 2 });
</script>
{#snippet s({ "a-b": ab, "c d": cd })}
	<button>{ab}{cd}</button>
{/snippet}
{@render s(v)}
