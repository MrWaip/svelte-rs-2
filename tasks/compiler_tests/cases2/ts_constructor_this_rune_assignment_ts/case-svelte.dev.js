App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#x;
		get x() {
			return $.get(this.#x);
		}
		set x(value) {
			$.set(this.#x, value, true);
		}
		constructor() {
			this.#x = $.tag($.state(0), "Counter.x");
		}
	}
	const c = new Counter();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.x));
	$.append($$anchor, p);
	return $.pop($$exports);
}
