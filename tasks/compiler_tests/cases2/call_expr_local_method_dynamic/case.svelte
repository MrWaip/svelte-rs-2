<script>
	let obj = { count: 0 };

	const get_count = () => obj.count;

	$effect(() => {
		obj.count += 1;
	});
</script>

<p>{obj.toString()}</p>
