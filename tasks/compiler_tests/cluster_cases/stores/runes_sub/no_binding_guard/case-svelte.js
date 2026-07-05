import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let foo = $.state(0);
	let bar = $.derived(() => $.get(foo) + 1);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(foo) ?? ""} ${$.get(bar) ?? ""}`));
	$.delegated("click", button, () => $.update(foo));
	$.append($$anchor, button);
}
$.delegate(["click"]);
