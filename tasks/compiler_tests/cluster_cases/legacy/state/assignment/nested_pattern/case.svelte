<script>
	let a = 0;
	let arr = [[1]];
	function update() {
		[[a]] = arr;
	}
</script>
<button on:click={update}>{a}</button>
