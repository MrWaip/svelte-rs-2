import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		#doubled = $.derived(() => $.get(this.#count) * 2);
		constructor() {
			console.log($.get(this.#doubled));
		}
	}
	let c = new Counter();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.display));
	$.append($$anchor, p);
	$.pop();
}
