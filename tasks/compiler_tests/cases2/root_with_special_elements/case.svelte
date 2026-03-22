<script>
	let count = $state(0);

	function handleScroll() {
		count++;
	}
</script>

<svelte:window onscroll={handleScroll} />

<div>Count: {count}</div>
