<script>
	let count = 0;
	function bump() {
		count += 1;
	}
</script>

<div title={[() => count]}></div>
<button onclick={bump}>go</button>
