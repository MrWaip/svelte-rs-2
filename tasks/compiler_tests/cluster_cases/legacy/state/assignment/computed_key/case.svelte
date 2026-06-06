<script>
	let a = 0;
	let key = "prop";
	let obj = { prop: 1 };
	function update() {
		({ [key]: a } = obj);
	}
</script>
<button on:click={update}>{a}</button>
