<script>
	class Counter {
		#a = $state();
		#b = $state({ val: -1 });
		#c = $state();
		constructor() {
			this.#a ||= { val: 0 };
			this.#b &&= { val: 0 };
			this.#c ??= { val: 0 };
		}
		get a() {
			return this.#a?.val;
		}
	}
	const counter = new Counter();
</script>

<p>{counter.a}</p>
