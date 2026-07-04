App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.#count");
		set count(x) {
			const local = 5;
			$.set(this.#count, local);
		}
	}
	const counter = new Counter();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, counter.count));
	$.delegated("click", button, function click() {
		return counter.count = 1;
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
