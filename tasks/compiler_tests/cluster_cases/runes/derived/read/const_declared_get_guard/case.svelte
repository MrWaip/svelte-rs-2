<script>
	let count = $state(0);
	const double = $derived(count * 2);
</script>

<button onclick={() => count++}>{double}</button>
