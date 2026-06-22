<script>
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	$: handler_a = flag ? handler_1 : handler_2;
</script>

<button on:click={handler_a}>x</button>
