<script>
	function box() {
		let state = $state(0);
		return {
			get value() {
				return state;
			}
		};
	}
	let b = box();
</script>
