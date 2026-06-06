<script>
	function load() { return { foo: 1 }; }
	const c = load();
	const { foo: x } = $derived(c);
</script>

<h1>{x}</h1>
