<script>
	let items = $state([1, 2, 3]);
</script>

<p>{JSON.stringify($state.snapshot(items))}</p>
