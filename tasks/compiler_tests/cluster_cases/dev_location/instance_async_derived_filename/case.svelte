<script>
	let { p } = $props();
	const total = $derived(await p);
</script>

<p>{total}</p>
