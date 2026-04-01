<script>
	let items = $state([1, 2, 3]);
	let [first, second, ...rest] = $derived(items);
</script>

<p>{first},{second}</p>
