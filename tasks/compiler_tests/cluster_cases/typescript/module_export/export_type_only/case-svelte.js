import * as $ from "svelte/internal/client";
export const K = 1;
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let count = $.state(0);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(count) ?? ""}1`));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
}
$.delegate(["click"]);
