import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let count = $.state(0);
	let double = $.derived(() => $.get(count) * 2);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(double)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
}
$.delegate(["click"]);
