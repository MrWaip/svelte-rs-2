<script>
	class Counter {
		count = $state(0);
		double = $derived(this.count * 2);
		inc() { this.count += 1; }
		get viaAlias() { const self = this; return self.count; }
	}
	const c = new Counter();
</script>
<button onclick={() => c.inc()}>{c.count} {c.double} {c.viaAlias}</button>
