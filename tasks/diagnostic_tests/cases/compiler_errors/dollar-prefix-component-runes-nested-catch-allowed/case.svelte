<script>
	let count = $state(0);

	function outer() {
		try {
			fetch('/');
		} catch ($$error) {
			console.log($$error);
		}
	}
</script>

{count}{outer}
