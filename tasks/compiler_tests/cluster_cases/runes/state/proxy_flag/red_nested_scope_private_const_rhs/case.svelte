<script>
	class Counter {
		#count = $state(0);
		set count(x) {
			const local = 5;
			this.#count = local;
		}
	}
	const counter = new Counter();
</script>

<button onclick={() => (counter.count = 1)}>{counter.count}</button>
