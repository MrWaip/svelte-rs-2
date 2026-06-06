<script>
	let { a = 0 } = $props();
	function f() { ({ a, ...rest } = { a: 1, b: 2 }); }
</script>
<button onclick={f}>{a}{rest.length}</button>
