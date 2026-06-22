<script lang="ts">
	class Counter {
		#count: number = $state(0);
		count2: number = $state<number>(0);
		#doubled: number = $derived(this.#count * 2);
		inc(): void { this.#count += 1; this.count2 += 1; }
		get value(): number { return this.#count; }
		get doubled(): number { return this.#doubled; }
	}
	const c = new Counter();
</script>
<button onclick={() => c.inc()}>{c.value} {c.count2} {c.doubled}</button>
