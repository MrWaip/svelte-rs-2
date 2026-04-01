<script>
	class Counter {
		#count = $state(0);
		#doubled = $derived(this.#count * 2);
		constructor() {
			console.log(this.#doubled);
		}
	}
	let c = new Counter();
</script>

<p>{c.display}</p>
