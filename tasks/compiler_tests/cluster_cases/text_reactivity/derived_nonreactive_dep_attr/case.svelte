<script>
	function load() { return { foo: 1 }; }
	const c = load();
	const x = $derived(c.foo);
</script>
<div title={x}></div>
