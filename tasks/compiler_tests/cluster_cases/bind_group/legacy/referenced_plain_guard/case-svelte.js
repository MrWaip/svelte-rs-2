import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<label>a <input type="checkbox"/></label>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let test = $.prop($$props, "test", 28, () => []);
	var label = root();
	var input = $.sibling($.child(label));
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	$.reset(label);
	$.bind_group(binding_group, [], input, test, test);
	$.append($$anchor, label);
}
