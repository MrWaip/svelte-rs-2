App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Box {
		#value = $.tag($.state(), "Box.#value");
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
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
