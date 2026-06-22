<script>
	let arr = [1, 2];
	function swap() {
		[arr[0], arr[1]] = [arr[1], arr[0]];
	}
</script>
<button on:click={swap}>{arr[0]}{arr[1]}</button>