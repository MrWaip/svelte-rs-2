import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>_</div> `, 1);
export default function App($$anchor) {
	let title = $.state(10);
	let title2 = $.state(12);
	$.update(title, -1);
	$.update_pre(title2);
	var fragment = root();
	var div = $.first_child(fragment);
	$.set_attribute(div, "attr", title++);
	var text = $.sibling(div);
	text.nodeValue = ` ${--title2 ?? ""}`;
	$.append($$anchor, fragment);
}
