import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let count = 0;
	const label = $.derived(() => {
		if (!count) return "a";
		return `a ${count}`;
	});
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(label)));
	$.append($$anchor, button);
}
