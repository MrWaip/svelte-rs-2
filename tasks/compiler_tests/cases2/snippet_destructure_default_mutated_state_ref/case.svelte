<script>
	let counter = $state(0);
	counter = 10;
</script>

{#snippet row({ values = [counter] })}
	<span>{values.length}</span>
{/snippet}

{@render row({})}
