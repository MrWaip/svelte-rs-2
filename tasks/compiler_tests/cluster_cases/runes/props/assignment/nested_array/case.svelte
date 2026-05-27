<script>
	let { a = 0, b = 0, c = 0, d = 0 } = $props();
	function f() { [[a, b], [c, d]] = [[1, 2], [3, 4]]; }
</script>
<button onclick={f}>{a}{b}{c}{d}</button>
