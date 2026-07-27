<script>
	let count = $state(0);

	function outer() {
		let { $$a } = source;
		return $$a;
	}
</script>

{count}{outer}
