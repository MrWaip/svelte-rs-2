<script>
	let arr = $state({ test: () => {} });
</script>

<!-- svelte-ignore state_snapshot_uncloneable -->
<div {...$state.snapshot(arr)}>a</div>
