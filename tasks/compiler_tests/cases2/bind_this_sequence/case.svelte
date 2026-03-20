<script>
	let ref = $state();

	function set(el) { ref = el; }
	function get() { return ref; }
</script>

<div bind:this={(el) => set(el), () => get()}></div>
