import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#total;
		get total() {
			return $.get(this.#total);
		}
		set total(value) {
			$.set(this.#total, value, true);
		}
		#count;
		constructor() {
			this.#count = $.state(0);
			this.#total = $.state(0);
		}
		bump() {
			$.update(this.#count);
			this.total++;
		}
		get count() {
			return $.get(this.#count);
		}
	}
	const counter = new Counter();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${counter.count ?? ""} ${counter.total ?? ""}`));
	$.delegated("click", button, () => counter.bump());
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
