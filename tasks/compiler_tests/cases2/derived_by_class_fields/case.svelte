<script>
	class Box {
		width = $state(2);
		height = $state(3);
		area = $derived.by(() => this.width * this.height);
		#double = $derived.by(() => this.area * 2);
		total;

		constructor() {
			this.total = $derived.by(() => this.area + 1);
		}

		get double() {
			return this.#double;
		}
	}

	let box = new Box();
</script>

<p>{box.area},{box.double},{box.total}</p>
