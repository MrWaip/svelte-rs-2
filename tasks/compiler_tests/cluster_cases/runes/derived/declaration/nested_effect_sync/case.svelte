<script>
	let count = $state(0);
	$effect(() => {
		let double = $derived(count * 2);
		console.log(double);
	});
</script>
<button onclick={() => count++}>Click</button>
