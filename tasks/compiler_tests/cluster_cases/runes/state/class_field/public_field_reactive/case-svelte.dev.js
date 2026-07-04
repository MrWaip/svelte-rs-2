App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[10, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.count");
		get count() {
			return $.get(this.#count);
		}
		set count(value) {
			$.set(this.#count, value, true);
		}
		#double = $.tag($.derived(() => this.count * 2), "Counter.double");
		get double() {
			return $.get(this.#double);
		}
		set double(value) {
			$.set(this.#double, value);
		}
		inc() {
			this.count += 1;
		}
		get viaAlias() {
			const self = this;
			return self.count;
		}
	}
	const c = new Counter();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${c.count ?? ""} ${c.double ?? ""} ${c.viaAlias ?? ""}`));
	$.delegated("click", button, function click() {
		return c.inc();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
