<script>
	let v = $state();
</script>
{#snippet s(x = { a: 1 })}
	<button>{x.a}</button>
{/snippet}
{@render s(v)}
