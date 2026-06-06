<script>
	let { a = 0 } = $props();
	function f() { [a, ...rest] = [1, 2, 3]; }
</script>
<button onclick={f}>{a}{rest.length}</button>
