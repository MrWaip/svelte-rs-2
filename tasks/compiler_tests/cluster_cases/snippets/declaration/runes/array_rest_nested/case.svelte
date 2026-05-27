<script>
	let v = $state([1, 2, 3]);
</script>
{#snippet s([a, ...[b, c]])}
	<button>{a}{b}{c}</button>
{/snippet}
{@render s(v)}
