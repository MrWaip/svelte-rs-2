import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let state = $.state(0);
	let derived = $.derived(() => $.get(state) + 1);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(state) ?? ""} ${$.get(derived) ?? ""}`));
	$.delegated("click", button, () => $.update(state));
	$.append($$anchor, button);
}
$.delegate(["click"]);
