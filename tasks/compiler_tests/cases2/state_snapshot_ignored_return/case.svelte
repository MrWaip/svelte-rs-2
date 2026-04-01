<script>
	let items = $state([1, 2, 3]);
	function getSnapshot() {
		// svelte-ignore state_snapshot_uncloneable
		return $state.snapshot(items);
	}
</script>
