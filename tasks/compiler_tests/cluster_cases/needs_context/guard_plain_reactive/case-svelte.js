import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let n = $.state(0);
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(n)));
	$.delegated("click", button, () => $.update(n));
	$.append($$anchor, button);
}
$.delegate(["click"]);
