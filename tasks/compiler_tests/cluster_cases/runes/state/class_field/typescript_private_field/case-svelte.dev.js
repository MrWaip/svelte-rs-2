App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#count = $.tag($.state(0), "Counter.#count");
		#count2 = $.tag($.state(0), "Counter.count2");
		get count2() {
			return $.get(this.#count2);
		}
		set count2(value) {
			$.set(this.#count2, value, true);
		}
		#doubled = $.tag($.derived(() => $.get(this.#count) * 2), "Counter.#doubled");
		inc() {
			$.set(this.#count, $.get(this.#count) + 1);
			this.count2 += 1;
		}
		get value() {
			return $.get(this.#count);
		}
		get doubled() {
			return $.get(this.#doubled);
		}
	}
	const c = new Counter();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${c.value ?? ""} ${c.count2 ?? ""} ${c.doubled ?? ""}`));
	$.delegated("click", button, function click() {
		return c.inc();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
