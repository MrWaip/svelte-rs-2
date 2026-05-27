<script>
	let { outer: [{ inner }] } = $state({ outer: [{ inner: 1 }] });
</script>
<button>{inner}</button>
