<script>
	let { a = 0, b = 0 } = $props();
	function f() { [a = 9, b = 9] = [1]; }
</script>
<button onclick={f}>{a}{b}</button>
