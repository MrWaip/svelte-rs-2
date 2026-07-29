<script>
	class Counter {
		#count = $state(0);

		increment() {
			this.#count += 1;
		}
	}

	const counter = new Counter();

	function handleError(e) {
		console.error(e, counter);
	}
</script>

<svelte:boundary onerror={handleError}>x</svelte:boundary>
