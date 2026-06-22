<script>
	let v = $state(1);
</script>
{#snippet s(a, b = 2)}
	<button>{a}{b}</button>
{/snippet}
{@render s(v)}
