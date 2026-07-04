App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[19, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#total;
		get total() {
			return $.get(this.#total);
		}
		set total(value) {
			$.set(this.#total, value, true);
		}
		#count;
		constructor() {
			this.#count = $.tag($.state(0), "Counter.#count");
			this.#total = $.tag($.state(0), "Counter.total");
		}
		bump() {
			$.update(this.#count);
			this.total++;
		}
		get count() {
			return $.get(this.#count);
		}
	}
	const counter = new Counter();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${counter.count ?? ""} ${counter.total ?? ""}`));
	$.delegated("click", button, function click() {
		return counter.bump();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
