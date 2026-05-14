import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	let double = $.derived(() => $.get(count) * 2);
	var $$exports = { get double() {
		return $.get(double);
	} };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(double)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
