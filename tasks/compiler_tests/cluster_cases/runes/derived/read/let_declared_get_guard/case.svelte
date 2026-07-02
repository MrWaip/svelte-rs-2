<script>
	let count = $state(0);
	let double = $derived(count * 2);
</script>

<button onclick={() => count++}>{double}</button>
