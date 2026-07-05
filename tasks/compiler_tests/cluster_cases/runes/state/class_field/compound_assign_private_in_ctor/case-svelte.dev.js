App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[18, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#a = $.tag($.state(), "Counter.#a");
		#b = $.tag($.state($.proxy({ val: -1 })), "Counter.#b");
		#c = $.tag($.state(), "Counter.#c");
		constructor() {
			$.set(this.#a, this.#a.v || { val: 0 }, true);
			$.set(this.#b, this.#b.v && { val: 0 }, true);
			$.set(this.#c, this.#c.v ?? { val: 0 }, true);
		}
		get a() {
			return $.get(this.#a)?.val;
		}
	}
	const counter = new Counter();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, counter.a));
	$.append($$anchor, p);
	return $.pop($$exports);
}
