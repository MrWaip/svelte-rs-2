<script>
	const k = "z";
	let { a = 0 } = $props();
	function f() { ({ [k]: a } = { z: 1 }); }
</script>
<button onclick={f}>{a}</button>
