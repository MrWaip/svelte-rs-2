App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Box {
		#value = $.tag($.state(0), "Box.value");
		get value() {
			return $.get(this.#value);
		}
		set value(value) {
			$.set(this.#value, value, true);
		}
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
