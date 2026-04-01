<script>
	let data = $state({ a: 1, b: 2 });
	let { a, b } = $derived.by(() => data);
</script>

<p>{a},{b}</p>
