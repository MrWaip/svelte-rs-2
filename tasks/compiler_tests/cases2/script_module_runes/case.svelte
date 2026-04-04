<script module>
	let shared = $state(0);
	let doubled = $derived(shared * 2);
</script>

<script>
	function increment() {
		shared++;
	}
</script>

<button onclick={increment}>{doubled}</button>
