import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div><input/></div>`);
export default function App($$anchor, $$props) {
	let foo = $.prop($$props, "foo", 12);
	var div = root();
	var input = $.child(div);
	$.remove_input_defaults(input);
	$.reset(div);
	$.bind_value(input, foo);
	$.append($$anchor, div);
}
