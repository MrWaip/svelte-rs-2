import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		get count() {
			return $.get(this.#count);
		}
		set count(val) {
			$.set(this.#count, val, true);
		}
	}
	const counter = new Counter();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, counter.count));
	$.delegated("click", button, () => counter.count++);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
