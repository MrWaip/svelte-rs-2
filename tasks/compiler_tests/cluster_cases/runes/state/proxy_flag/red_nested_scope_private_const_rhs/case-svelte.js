import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		set count(x) {
			const local = 5;
			$.set(this.#count, local);
		}
	}
	const counter = new Counter();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, counter.count));
	$.delegated("click", button, () => counter.count = 1);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
