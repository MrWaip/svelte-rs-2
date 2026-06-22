import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Counter {
		#count = $.state(0);
		#count2 = $.state(0);
		get count2() {
			return $.get(this.#count2);
		}
		set count2(value) {
			$.set(this.#count2, value, true);
		}
		#doubled = $.derived(() => $.get(this.#count) * 2);
		inc() {
			$.set(this.#count, $.get(this.#count) + 1);
			this.count2 += 1;
		}
		get value() {
			return $.get(this.#count);
		}
		get doubled() {
			return $.get(this.#doubled);
		}
	}
	const c = new Counter();
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${c.value ?? ""} ${c.count2 ?? ""} ${c.doubled ?? ""}`));
	$.delegated("click", button, () => c.inc());
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
