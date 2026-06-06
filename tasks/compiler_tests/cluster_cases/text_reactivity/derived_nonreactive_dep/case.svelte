<script>
	function load() { return { foo: 1 }; }
	const c = load();
	const x = $derived(c.foo);
</script>

<h1>{x}</h1>
