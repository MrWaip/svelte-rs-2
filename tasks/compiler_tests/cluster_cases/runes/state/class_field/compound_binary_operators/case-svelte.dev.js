App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[17, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#n = $.tag($.state(0), "Counter.#n");
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
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, c.n));
	$.delegated("click", button, function click() {
		return c.bump();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
