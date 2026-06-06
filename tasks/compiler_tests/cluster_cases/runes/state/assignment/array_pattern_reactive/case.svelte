<script>
	let a = $state(0);
	let b = $state(0);
	function update() {
		[a, b] = [1, 2];
	}
</script>
<button onclick={update}>{a}{b}</button>
