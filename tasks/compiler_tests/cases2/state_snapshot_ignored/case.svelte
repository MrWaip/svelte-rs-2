<script>
	let items = $state([1, 2, 3]);
	// svelte-ignore state_snapshot_uncloneable
	let snap = $state.snapshot(items);
</script>
