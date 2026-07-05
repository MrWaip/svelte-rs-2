import * as $ from "svelte/internal/client";
export default function App($$anchor) {
	class Box {
		#value = $.state(0);
		get value() {
			return $.get(this.#value);
		}
		set value(value) {
			$.set(this.#value, value, true);
		}
	}
}
