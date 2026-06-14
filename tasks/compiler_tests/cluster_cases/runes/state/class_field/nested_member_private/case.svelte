<script>
	class Store {
		#data = $state({ n: 0 });
		get data() {
			return this.#data;
		}
		inc() {
			this.#data.n++;
		}
	}
	const store = new Store();
</script>

<button onclick={() => store.inc()}>{store.data.n}</button>
