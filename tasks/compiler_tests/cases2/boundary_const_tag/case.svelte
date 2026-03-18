<script>
	let items = $state(["a", "b"]);
</script>

<svelte:boundary>
	{@const count = items.length}
	<p>Count: {count}</p>
</svelte:boundary>
