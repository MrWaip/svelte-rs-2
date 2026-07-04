App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.#count");
		get count() {
			return $.get(this.#count);
		}
		set count(val) {
			$.set(this.#count, val, true);
		}
	}
	const counter = new Counter();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, counter.count));
	$.delegated("click", button, function click() {
		return counter.count++;
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
