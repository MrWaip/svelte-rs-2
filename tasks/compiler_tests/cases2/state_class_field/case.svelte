<script>
	class Counter {
		count = $state(0);
		increment() {
			this.count += 1;
		}
	}
	let c = new Counter();
</script>

<p>{c.count}</p>
