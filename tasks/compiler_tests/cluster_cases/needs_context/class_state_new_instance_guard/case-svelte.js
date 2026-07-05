import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	class Box {
		#value = $.state(0);
		get value() {
			return $.get(this.#value);
		}
		set value(value) {
			$.set(this.#value, value, true);
		}
	}
	const box = new Box();
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, box.value));
	$.delegated("click", button, () => box.value++);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
