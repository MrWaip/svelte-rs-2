App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.#count");
		get value() {
			return $.get(this.#count);
		}
	}
	let c = new Counter();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.value));
	$.append($$anchor, p);
	return $.pop($$exports);
}
