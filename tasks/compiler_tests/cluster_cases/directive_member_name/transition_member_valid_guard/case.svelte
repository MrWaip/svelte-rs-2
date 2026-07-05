<script>
	const fn = (node, options) => ({});
	let obj = $derived({ inner: fn });
</script>

<div transition:obj.inner></div>
