import * as $ from "svelte/internal/client";
var root = $.from_html(`<p></p> <p></p>`, 1);
export default function App($$anchor) {
	let maybeNull = null;
	let maybeUndefined = undefined;
	var fragment = root();
	var p = $.first_child(fragment);
	var p_1 = $.sibling(p, 2);
	$.append($$anchor, fragment);
}
