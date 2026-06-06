<script>
	let a = 0;
	let arr = [];
	function update() {
		[a = 5] = arr;
	}
</script>
<button on:click={update}>{a}</button>
