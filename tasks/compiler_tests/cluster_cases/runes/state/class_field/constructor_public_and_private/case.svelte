<script>
	class Counter {
		#count;
		constructor() {
			this.#count = $state(0);
			this.total = $state(0);
		}
		bump() {
			this.#count++;
			this.total++;
		}
		get count() {
			return this.#count;
		}
	}
	const counter = new Counter();
</script>

<button onclick={() => counter.bump()}>{counter.count} {counter.total}</button>
