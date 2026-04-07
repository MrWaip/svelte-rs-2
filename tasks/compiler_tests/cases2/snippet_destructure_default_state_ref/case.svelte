<script>
	let counter = $state(0);
</script>

{#snippet row({ values = [counter] })}
	<span>{values.length}</span>
{/snippet}

{@render row({})}
