import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const initial = 0;
	let value = $.state(initial);
	function bump() {
		$.set(value, $.get(value) + 1);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(value)));
	$.delegated("click", button, bump);
	$.append($$anchor, button);
}
$.delegate(["click"]);
