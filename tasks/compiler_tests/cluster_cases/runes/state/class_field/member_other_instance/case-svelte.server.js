import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		class Box {
			#value;
			constructor(value) {
				this.#value = value;
			}
			get value() {
				return this.#value;
			}
			swap(other) {
				const value = this.#value;
				this.#value = other.value;
				other.#value = value;
			}
		}
		const a = new Box(42);
		const b = new Box(99);
		a.swap(b);
	});
}
