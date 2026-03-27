<script>
	let [x, y] = $state([1, 2]);
	let { a, b } = $state({ a: 1, b: 2 });
	let { c = 10, d = 20 } = $state({});
	let { e: { f } } = $state({ e: { f: 1 } });
	let { g, ...rest } = $state({ g: 1, h: 2, i: 3 });
	x += 1;
	a += 1;
	c += 1;
</script>

<p>{x} {y} {a} {b} {c} {d} {f} {g}</p>
