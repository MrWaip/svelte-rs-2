<script>
	let { a = 0, b = 0 } = $props();
	function f() { [a, b] = [1, 2]; }
</script>
<button onclick={f}>{a}{b}</button>
