<script>
	let v = $state([1, 2]);
</script>
{#snippet s([a, b])}
	<button>{a}{b}</button>
{/snippet}
{@render s(v)}
