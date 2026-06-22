import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#a = $.state();
		#b = $.state($.proxy({ val: -1 }));
		#c = $.state();
		constructor() {
			$.set(this.#a, this.#a.v || { val: 0 }, true);
			$.set(this.#b, this.#b.v && { val: 0 }, true);
			$.set(this.#c, this.#c.v ?? { val: 0 }, true);
		}
		get a() {
			return $.get(this.#a)?.val;
		}
	}
	const counter = new Counter();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, counter.a));
	$.append($$anchor, p);
	$.pop();
}
