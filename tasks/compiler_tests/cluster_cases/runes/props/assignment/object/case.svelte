<script>
	let { a = 0, b = 0 } = $props();
	function f() { ({ a, b } = { a: 1, b: 2 }); }
</script>
<button onclick={f}>{a}{b}</button>
