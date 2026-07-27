<script>
	let count = 0;
	$: $count = 1;
</script>

<button onclick={() => count = 5}>set</button>
