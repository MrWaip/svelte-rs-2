<script>
	class Counter {
		#n = $state(0);
		bump() {
			const self = this;
			self.#n += 1;
		}
		get n() {
			return this.#n;
		}
	}
	const c = new Counter();
</script>

<button onclick={() => c.bump()}>{c.n}</button>
