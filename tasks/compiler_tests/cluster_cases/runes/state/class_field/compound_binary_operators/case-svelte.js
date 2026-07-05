import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#n = $.state(0);
		bump() {
			$.set(this.#n, $.get(this.#n) + 1);
			$.set(this.#n, $.get(this.#n) << 2);
			$.set(this.#n, $.get(this.#n) >>> 1);
			$.set(this.#n, $.get(this.#n) & 6);
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
