import * as $ from "svelte/internal/client";
var root = $.from_html(`<input/> <button>s</button> `, 1);
export default function App($$anchor) {
	let s = $.state(0);
	let d = $.derived(() => $.get(s) * 2);
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var button = $.sibling(input, 2);
	var text = $.sibling(button);
	$.template_effect(() => $.set_text(text, ` ${$.get(d) ?? ""}`));
	$.bind_value(input, () => $.get(d), ($$value) => $.set(d, $$value));
	$.delegated("click", button, () => $.update(s));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
