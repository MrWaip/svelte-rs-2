<script>
	class Counter {
		#count = $state(0);
		get count() {
			return this.#count;
		}
		reset() {
			this.#count = 5;
		}
	}
	const counter = new Counter();
</script>

<button onclick={() => counter.reset()}>{counter.count}</button>
