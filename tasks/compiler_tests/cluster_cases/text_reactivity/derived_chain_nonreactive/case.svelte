<script>
	function load() { return 1; }
	const a = $derived(load());
	const x = $derived(a + 1);
</script>
<h1>{x}</h1>
