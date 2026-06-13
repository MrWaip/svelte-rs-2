<script>
	let v = $state(false);
</script>
{#snippet s(x = true)}
	<button>{x}</button>
{/snippet}
{@render s(v)}
