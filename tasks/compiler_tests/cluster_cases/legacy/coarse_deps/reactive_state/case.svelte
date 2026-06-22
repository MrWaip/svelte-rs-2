<script>
	export let a;
	let s = 0;
	function inc() { s++; }
	$: {
		a;
		s;
	}
</script>
