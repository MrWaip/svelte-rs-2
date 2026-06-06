<script>
	let a = $state(0);
	let obj = { x: 0 };
	let arr = [1, 2];
	function update() {
		[obj.x, a] = arr;
	}
</script>
<button onclick={update}>{a}</button>
