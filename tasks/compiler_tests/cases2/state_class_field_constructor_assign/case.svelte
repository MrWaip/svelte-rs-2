<script>
	class Counter {
		count = $state(0);

		constructor(initial) {
			this.count = initial;
		}
	}
	let c = new Counter(10);
</script>

<p>{c.count}</p>
