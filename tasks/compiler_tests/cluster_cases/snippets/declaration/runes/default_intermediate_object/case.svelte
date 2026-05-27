<script>
	let v = $state({});
</script>
{#snippet s({ p: { a } = {} })}
	<button>{a}</button>
{/snippet}
{@render s(v)}
