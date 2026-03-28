<script>
	class Timer {
		#elapsed = $state(0);
		tick() {
			this.#elapsed += 1;
		}
		get display() {
			return this.#elapsed;
		}
	}
	let t = new Timer();
</script>

<p>{t.display}</p>
