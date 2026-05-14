import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#x = $.state(0);
		get x() {
			return $.get(this.#x);
		}
		set x(value) {
			$.set(this.#x, value, true);
		}
	}
	const c = new Counter();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.x));
	$.append($$anchor, p);
	$.pop();
}
