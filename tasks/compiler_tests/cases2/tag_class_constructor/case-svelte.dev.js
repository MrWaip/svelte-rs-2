App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count;
		get count() {
			return $.get(this.#count);
		}
		set count(value) {
			$.set(this.#count, value, true);
		}
		constructor() {
			this.#count = $.tag($.state(0), "Counter.count");
		}
	}
	let c = new Counter();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.count));
	$.append($$anchor, p);
	return $.pop($$exports);
}
