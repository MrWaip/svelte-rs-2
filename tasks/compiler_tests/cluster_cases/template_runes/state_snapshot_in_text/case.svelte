<script>
	let obj = $state({ a: 1 });
</script>

<p>{JSON.stringify($state.snapshot(obj))}</p>
