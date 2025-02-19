import * as $ from "svelte/internal/client";
var root = $.template(`<input> <input>`, 1);
export default function App($$anchor) {
	let value = $.state("");
	var fragment = root();
	var input = $.first_child(fragment);
	var input_1 = $.sibling(input, 2);
	$.bind_value(input, ($$value) => $.set(value, $.proxy($$value)));
	$.bind_value(input_1, ($$value) => $.set(value, $.proxy($$value)));
	$.append($$anchor, fragment);
}
