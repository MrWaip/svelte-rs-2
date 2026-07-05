<script>
	let count = $state(0);
	function read() {
		let double = $derived(count * 2);
		return double;
	}
</script>
<button onclick={() => count++}>{read()}</button>
