App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(), "Counter.#count");
		constructor() {
			const instance = this;
			$.set(instance.#count, 1);
		}
		get count() {
			return $.get(this.#count);
		}
		get count2() {
			const instance = this;
			return $.get(instance.#count);
		}
	}
	const counter = new Counter();
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
