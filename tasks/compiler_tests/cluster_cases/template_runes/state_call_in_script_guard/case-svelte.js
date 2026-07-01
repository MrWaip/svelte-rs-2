import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let x = $.proxy({ a: 1 });
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, x.a));
	$.delegated("click", button, () => x.a++);
	$.append($$anchor, button);
}
$.delegate(["click"]);
