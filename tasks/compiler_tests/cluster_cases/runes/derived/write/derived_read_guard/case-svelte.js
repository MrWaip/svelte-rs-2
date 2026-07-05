import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>s</button> `, 1);
export default function App($$anchor) {
	let s = $.state(0);
	let d = $.derived(() => $.get(s) * 2);
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(() => $.set_text(text, ` ${$.get(d) ?? ""}`));
	$.delegated("click", button, () => $.update(s));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
