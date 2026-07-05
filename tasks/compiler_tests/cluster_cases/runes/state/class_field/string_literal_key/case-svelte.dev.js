App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	class Toggle {
		#aria_pressed = $.tag($.state(false), "Toggle.aria-pressed");
		get "aria-pressed"() {
			return $.get(this.#aria_pressed);
		}
		set "aria-pressed"(value) {
			$.set(this.#aria_pressed, value, true);
		}
		toggle() {
			this["aria-pressed"] = !this["aria-pressed"];
		}
	}
	const toggle = new Toggle();
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, toggle["aria-pressed"]));
	$.delegated("click", button, function click() {
		return toggle.toggle();
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
