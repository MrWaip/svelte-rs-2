<script>
	let arr = $state({ test: 1 });
</script>

<div {...$state.snapshot(arr)}>a</div>
