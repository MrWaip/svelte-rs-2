<script>
	class Box {
		width = $state(2);
		height = $state(3);
		area = $derived.by(() => this.width * this.height);

		constructor() {
			this.total = $derived.by(() => this.area + 1);
		}
	}

	let box = new Box();
</script>

<p>{box.area},{box.total}</p>
