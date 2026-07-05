<script>
	let count = $state(0);
	var double = $derived.by(() => count * 2);
</script>

<button onclick={() => count++}>{double}</button>
