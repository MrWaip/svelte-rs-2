<script>
	class Counter {
		constructor() {
			this.count = $state(0);
		}
	}
	let c = new Counter();
</script>

<p>{c.count}</p>
