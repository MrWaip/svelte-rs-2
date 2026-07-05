App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.#count");
		#doubled = $.tag($.derived(() => $.get(this.#count) * 2), "Counter.#doubled");
		constructor() {
			console.log(...$.log_if_contains_state("log", $.get(this.#doubled)));
		}
	}
	let c = new Counter();
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, c.display));
	$.append($$anchor, p);
	return $.pop($$exports);
}
