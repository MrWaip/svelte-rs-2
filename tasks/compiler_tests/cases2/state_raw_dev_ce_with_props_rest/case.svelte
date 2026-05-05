<script>
	let { x = 0, ...rest } = $props();
	let rawData = $state.raw({ a: 1, b: 2 });
	let snapshot = $state.snapshot(rawData);
</script>

<p>{x} {snapshot.a}</p>
