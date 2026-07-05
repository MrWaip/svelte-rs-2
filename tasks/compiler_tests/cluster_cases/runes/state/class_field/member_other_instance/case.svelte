<script>
	class Box {
		#value = $state();
		constructor(value) {
			this.#value = value;
		}
		get value() {
			return this.#value;
		}
		swap(other) {
			const value = this.#value;
			this.#value = other.value;
			other.#value = value;
		}
	}
	const a = new Box(42);
	const b = new Box(99);
	a.swap(b);
</script>
