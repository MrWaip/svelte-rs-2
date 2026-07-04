import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count;
		get count() {
			return $.get(this.#count);
		}
		set count(value) {
			$.set(this.#count, value, true);
		}
		constructor() {
			this.#count = $.state(0);
		}
	}
	let c = new Counter();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.count));
	$.append($$anchor, p);
	$.pop();
}
