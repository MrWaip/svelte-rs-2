import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let value = $.state(0);
	function clamp(x) {
		$.set(value, Math.min(100, +x), true);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, () => clamp(5));
	$.append($$anchor, button);
}
$.delegate(["click"]);
