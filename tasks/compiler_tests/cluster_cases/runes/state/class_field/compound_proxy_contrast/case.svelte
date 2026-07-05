<script>
	class Box {
		#a = $state({ val: 0 });
		#b = $state(0);
		mix() {
			this.#a ??= { val: 1 };
			this.#b += 1;
		}
		get a() {
			return this.#a?.val;
		}
		get b() {
			return this.#b;
		}
	}
	const box = new Box();
</script>

<p>{box.a} {box.b}</p>
