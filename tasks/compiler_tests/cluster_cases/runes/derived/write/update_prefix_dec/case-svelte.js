import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button> <button>s</button> `, 1);
export default function App($$anchor) {
	let s = $.state(0);
	let d = $.derived(() => $.get(s) * 2);
	var fragment = root();
	var button = $.first_child(fragment);
	var button_1 = $.sibling(button, 2);
	var text = $.sibling(button_1);
	$.template_effect(() => $.set_text(text, ` ${$.get(d) ?? ""}`));
	$.delegated("click", button, () => $.update_pre(d, -1));
	$.delegated("click", button_1, () => $.update(s));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
