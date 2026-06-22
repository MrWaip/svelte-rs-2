<script>
	let s = $state(0);
	function inc() { s++; }
	const x = $derived(s + 1);
</script>
<h1>{x}</h1>
