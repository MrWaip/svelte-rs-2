<script>
	let foo = $state(10);
	function wrap() {
		return {
			set(state) {
				foo = state;
			}
		};
	}
	let w = wrap();
</script>
