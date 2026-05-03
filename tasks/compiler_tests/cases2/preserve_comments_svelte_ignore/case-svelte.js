import * as $ from "svelte/internal/client";
var root = $.from_html(`<!-- svelte-ignore a11y_autofocus --> <input/>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.first_child(fragment);
	var input = $.sibling(node, 2);
	$.autofocus(input, true);
	$.append($$anchor, fragment);
}
