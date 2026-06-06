<script>
	let v = $state([1]);
</script>
{#snippet s([a = 10, b = 20])}
	<button>{a}{b}</button>
{/snippet}
{@render s(v)}
