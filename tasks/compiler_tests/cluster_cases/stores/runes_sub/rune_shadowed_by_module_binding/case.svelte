<script module>
	const data = $state(0);
	const derived = $derived(data + 1);
	const state = { data, derived };
</script>

<script>
	$inspect(state);
</script>

{state.data}
