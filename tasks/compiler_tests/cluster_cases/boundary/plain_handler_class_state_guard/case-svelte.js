import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		increment() {
			$.set(this.#count, $.get(this.#count) + 1);
		}
	}
	const counter = new Counter();
	function handleError(e) {
		console.error(e, counter);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { onerror: handleError }, ($$anchor) => {
		$.next();
		var text = $.text("x");
		$.append($$anchor, text);
	});
	$.append($$anchor, fragment);
	$.pop();
}
