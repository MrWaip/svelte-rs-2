<script>
	let n = $state(0);
</script>

<p>{(() => {
	$effect.root(() => {
		n;
	});
	return n;
})()}</p>
