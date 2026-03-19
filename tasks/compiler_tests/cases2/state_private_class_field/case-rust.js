import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		get value() {
			return $.get(this.#count);
		}
	}
	let c = new Counter();
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.value));
	$.append($$anchor, p);
	$.pop();
}
