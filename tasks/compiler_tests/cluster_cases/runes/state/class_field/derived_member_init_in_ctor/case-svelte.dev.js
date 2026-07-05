App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[22, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Counter {
		#doubled;
		get doubled() {
			return $.get(this.#doubled);
		}
		set doubled(value) {
			$.set(this.#doubled, value);
		}
		#count;
		constructor(initial) {
			this.#count = $.tag($.state($.proxy(initial)), "Counter.#count");
			this.#doubled = $.tag($.derived(() => $.get(this.#count) * 2), "Counter.#doubled");
		}
		increment = () => {
			$.update(this.#count);
		};
	}
	const counter = new Counter(10);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, counter.doubled));
	$.delegated("click", button, function click() {
		return counter.increment();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
