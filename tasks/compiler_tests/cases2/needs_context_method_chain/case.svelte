<script>
	let items = $state([]);

	let total = $derived.by(() => {
		return items.filter(Boolean).length;
	});
</script>

<p>{total}</p>
