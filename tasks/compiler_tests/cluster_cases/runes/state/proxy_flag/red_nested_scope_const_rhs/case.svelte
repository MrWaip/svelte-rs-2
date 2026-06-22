<script>
	let value = $state(0);
	const handler = () => {
		const local = 5;
		value = local;
	};
</script>

<button onclick={handler}>{value}</button>
