<script>
	let handler = $state(() => {});
	$effect(() => {
		handler = () => console.log('x');
	});
</script>

<button onclick={handler}>x</button>
