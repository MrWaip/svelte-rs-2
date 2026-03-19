<script>
	class Counter {
		#count = $state(0);
		get value() {
			return this.#count;
		}
	}
	let c = new Counter();
</script>

<p>{c.value}</p>
