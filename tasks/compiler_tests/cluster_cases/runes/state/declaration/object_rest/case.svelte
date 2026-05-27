<script>
	let { a, ...rest } = $state({ a: 1, b: 2, c: 3 });
</script>
<button>{a}{rest.b}</button>
