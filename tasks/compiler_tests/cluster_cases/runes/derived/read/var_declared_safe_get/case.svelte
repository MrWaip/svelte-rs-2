<script>
	console.log(double);
	let count = $state(0);
	var double = $derived(count * 2);
</script>

<button onclick={() => count++}>{double}</button>
