<script>
	const foo = $effect.tracking();
	let bar = $state(false);
	$effect.pre(() => {
		bar = $effect.tracking();
	});
</script>

<p>{foo} {bar}</p>
