<script>
	let arr = $state([1, 2, 3]);
	await Promise.resolve();
	let [a, ...rest] = $derived(arr);
	let { x = 1, ...others } = $derived({ x: 2 });
</script>
<p>{a} {rest.length} {x} {others.y}</p>
