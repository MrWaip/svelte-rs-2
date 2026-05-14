<script>
	let { manager } = $props();
	let { a, b } = $derived(manager);
</script>

<p>{a},{b}</p>
