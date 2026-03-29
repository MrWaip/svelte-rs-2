import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		get count() {
			return $.get(this.#count);
		}
		set count(value) {
			$.set(this.#count, value, true);
		}
		constructor(initial) {
			this.count = initial;
		}
	}
	let c = new Counter(10);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.count));
	$.append($$anchor, p);
	$.pop();
}
