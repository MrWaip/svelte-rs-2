import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
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
		#double = $.derived(() => this.count * 2);
		get double() {
			return $.get(this.#double);
		}
		set double(value) {
			$.set(this.#double, value);
		}
		inc() {
			this.count += 1;
		}
		get viaAlias() {
			const self = this;
			return self.count;
		}
	}
	const c = new Counter();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${c.count ?? ""} ${c.double ?? ""} ${c.viaAlias ?? ""}`));
	$.delegated("click", button, () => c.inc());
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
