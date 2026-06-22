<script>
	let flag = $state(true);
	const handler_1 = () => {};
	const handler_2 = () => {};
	let handler = $derived(flag ? handler_1 : handler_2);
</script>

<button onclick={handler}>x</button>
