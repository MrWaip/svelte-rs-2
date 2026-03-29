<script>
	let value = 42;

	const double = (n) => n * 2;

	const get_value = () => value;
</script>

<p>{double(get_value())}</p>
