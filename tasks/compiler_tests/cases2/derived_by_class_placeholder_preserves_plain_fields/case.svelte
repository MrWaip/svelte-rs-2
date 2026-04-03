<script>
	class Box {
		width = $state(2);
		height = $state(3);
		area = $derived.by(() => this.width * this.height);
		total;
		stable = 1;

		constructor() {
			this.total = $derived.by(() => this.area + this.stable);
		}
	}

	let box = new Box();
</script>

<p>{box.area},{box.total},{box.stable}</p>
