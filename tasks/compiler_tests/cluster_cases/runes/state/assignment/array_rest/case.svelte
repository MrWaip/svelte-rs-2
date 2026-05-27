<script>
	let a = $state(0);
	let b = $state(0);
	let arr = [1, 2, 3];
	function update() {
		[a, ...b] = arr;
	}
</script>
<button onclick={update}>{a}{b}</button>
