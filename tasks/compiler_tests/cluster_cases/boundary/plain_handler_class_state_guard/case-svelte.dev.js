App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.#count");
		increment() {
			$.set(this.#count, $.get(this.#count) + 1);
		}
	}
	const counter = new Counter();
	function handleError(e) {
		console.error(...$.log_if_contains_state("error", e, counter));
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { onerror: handleError }, ($$anchor) => {
		$.next();
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
