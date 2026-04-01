<script>
	class Counter {
		#count = $state(0);
		constructor() {
			console.log(this.#count);
		}
	}
	let c = new Counter();
</script>

<p>{c}</p>
