import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Toggle {
		#aria_pressed = $.state(false);
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
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, toggle["aria-pressed"]));
	$.delegated("click", button, () => toggle.toggle());
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
