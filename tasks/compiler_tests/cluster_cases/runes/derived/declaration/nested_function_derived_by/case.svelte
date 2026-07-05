<script>
	let count = $state(0);
	$effect(() => {
		let double = $derived.by(() => count * 2);
		console.log(double);
	});
</script>
<button onclick={() => count++}>Click</button>
