import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let value = $.state(0);
	const min = 2;
	function clamp() {
		$.set(value, min);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, clamp);
	$.append($$anchor, button);
}
$.delegate(["click"]);
