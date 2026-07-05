App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[11, 0]]);
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
		bump() {
			this.value++;
		}
	}
	const box = new Box();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, box.value));
	$.delegated("click", button, function click() {
		return box.bump();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
