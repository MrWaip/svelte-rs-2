<script>
	class Counter {
		#doubled;
		get doubled() {
			return this.#doubled;
		}
		set doubled(value) {
			this.#doubled = value;
		}
		#count;
		constructor(initial) {
			this.#count = $state(initial);
			this.#doubled = $derived(this.#count * 2);
		}
		increment = () => {
			this.#count++;
		};
	}
	const counter = new Counter(10);
</script>

<button onclick={() => counter.increment()}>{counter.doubled}</button>
