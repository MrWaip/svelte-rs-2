<script>
	let coords = $state({ x: 0, y: 0 });
	let { x, y } = $derived(coords);
</script>

<p>{x},{y}</p>
