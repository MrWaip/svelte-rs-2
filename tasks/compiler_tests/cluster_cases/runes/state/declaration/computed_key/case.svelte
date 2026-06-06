<script>
	const k = "z";
	let { [k]: v } = $state({ z: 1 });
</script>
<button>{v}</button>
