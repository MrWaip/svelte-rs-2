import * as $ from "svelte/internal/client";
var root = $.template(`<div>text only</div> <div></div>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var div = $.sibling($.first_child(fragment), 2);
	div.textContent = interpolation;
	$.append($$anchor, fragment);
}
