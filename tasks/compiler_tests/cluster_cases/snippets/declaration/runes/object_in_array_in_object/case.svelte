<script>
	let v = $state({ outer: [{ inner: 1 }] });
</script>
{#snippet s({ outer: [{ inner }] })}
	<button>{inner}</button>
{/snippet}
{@render s(v)}
