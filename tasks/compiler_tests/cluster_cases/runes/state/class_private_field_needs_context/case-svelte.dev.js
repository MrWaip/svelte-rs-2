App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = 0;
		constructor(initial) {
			this.#count = initial;
		}
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
