<script>
	let a = $state(0);
	let arr = [];
	function update() {
		[{ a } = {}] = arr;
	}
</script>
<button onclick={update}>{a}</button>
