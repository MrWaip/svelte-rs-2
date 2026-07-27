import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button>go</button>`, 1);
export default function App($$anchor) {
	let value = $.state("x");
	var fragment = root();
	var div = $.first_child(fragment);
	var button = $.sibling(div, 2);
	$.template_effect(() => $.set_attribute(div, "foo", `a${$.get(value) ?? ""}`));
	$.delegated("click", button, () => $.set(value, $.get(value) + "!"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
