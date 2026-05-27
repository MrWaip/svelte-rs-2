<script>
	let v = $state([[1, 2], [3, 4]]);
</script>
{#snippet s([[a, b], [c, d]])}
	<button>{a}{b}{c}{d}</button>
{/snippet}
{@render s(v)}
