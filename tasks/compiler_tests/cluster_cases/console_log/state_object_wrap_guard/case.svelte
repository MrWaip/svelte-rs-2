<script>
	let obj = $state({ a: 1 });
	function report() {
		console.log(obj);
	}
</script>

<button onclick={report}>go</button>
