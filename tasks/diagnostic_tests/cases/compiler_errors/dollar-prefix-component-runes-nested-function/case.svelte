<script>
	let count = $state(0);

	function outer() {
		function inner() {
			let $$a = 1;
			return $$a;
		}
		return inner;
	}
</script>

{count}{outer}
