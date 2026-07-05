import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#n = $.state(0);
		bump() {
			const self = this;
			$.set(self.#n, $.get(self.#n) + 1);
		}
		get n() {
			return $.get(this.#n);
		}
	}
	const c = new Counter();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, c.n));
	$.delegated("click", button, () => c.bump());
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
