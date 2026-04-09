<script>
	let { pair: [a, b] } = $state({ pair: [1, 2] });
	a = 10;
	b = 20;
</script>

<p>{a} {b}</p>
