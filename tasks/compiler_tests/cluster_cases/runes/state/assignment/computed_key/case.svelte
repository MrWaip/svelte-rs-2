<script>
	let a = $state(0);
	let key = "prop";
	let obj = { prop: 1 };
	function update() {
		({ [key]: a } = obj);
	}
</script>
<button onclick={update}>{a}</button>
