<script>
	let elem = $state.raw();
	$effect(() => {
		console.log(elem);
	});
</script>

<div bind:this={elem}></div>
