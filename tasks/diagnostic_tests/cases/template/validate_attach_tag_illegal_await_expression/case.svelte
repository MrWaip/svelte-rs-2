<script>
	async function load() {
		return (_node) => {};
	}

	function attach(_arg) {
		return (_node) => {};
	}
</script>

<div {@attach attach(await load())}></div>
