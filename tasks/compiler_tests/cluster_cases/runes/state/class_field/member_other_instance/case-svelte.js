import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Box {
		#value = $.state();
		constructor(value) {
			$.set(this.#value, value, true);
		}
		get value() {
			return $.get(this.#value);
		}
		swap(other) {
			const value = $.get(this.#value);
			$.set(this.#value, other.value, true);
			$.set(other.#value, value, true);
		}
	}
	const a = new Box(42);
	const b = new Box(99);
	a.swap(b);
	$.pop();
}
