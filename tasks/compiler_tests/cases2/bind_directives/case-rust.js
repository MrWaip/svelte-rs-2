import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input>`, 1);
export default function App($$anchor) {
	let value = "";
	let name = "";
	var fragment = root();
	var input = $.first_child(fragment);
	var input_1 = $.sibling(input, 2);
	$.append($$anchor, fragment);
}
