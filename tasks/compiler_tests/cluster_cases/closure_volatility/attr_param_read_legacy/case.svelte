<script>
	let count = 0;
	function bump() {
		count += 1;
	}
</script>

<div title={[(x) => x]}></div>
<button onclick={bump}>go</button>
