import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <button>go</button>`, 1);
export default function App($$anchor) {
	let count = $.state(0);
	var fragment = root();
	var div = $.first_child(fragment);
	var button = $.sibling(div, 2);
	$.template_effect(($0) => $.set_attribute(div, "title", $0), [() => String($.get(count))]);
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
