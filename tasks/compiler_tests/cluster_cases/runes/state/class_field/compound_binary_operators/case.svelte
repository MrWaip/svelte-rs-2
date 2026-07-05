<script>
	class Counter {
		#n = $state(0);
		bump() {
			this.#n += 1;
			this.#n <<= 2;
			this.#n >>>= 1;
			this.#n &= 6;
		}
		get n() {
			return this.#n;
		}
	}
	const c = new Counter();
</script>

<button onclick={() => c.bump()}>{c.n}</button>
