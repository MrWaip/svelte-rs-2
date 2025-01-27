import * as $ from "svelte/internal/client";
var on_click = (_, title) => $.update(title);
var root = $.template(`<div>_</div> `, 1);
export default function App($$anchor) {
	let title = $.state(10);
	var fragment = root();
	var div = $.first_child(fragment);
	div.__click = [on_click, title];
	var text = $.sibling(div, 1, true);
	$.template_effect(() => $.set_text(text, $.get(title)));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
