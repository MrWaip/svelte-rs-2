<script>
	let a = $state(0);
	let arr = [{ a: 1 }];
	function update() {
		[{ a }] = arr;
	}
</script>
<button onclick={update}>{a}</button>
