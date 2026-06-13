<script>
	let v = $state(1);
</script>
{#snippet s(x)}
	<button>{x}</button>
{/snippet}
{@render s(v)}
