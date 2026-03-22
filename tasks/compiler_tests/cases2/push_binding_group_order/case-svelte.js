import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/> <input type="radio"/> <p> </p>`, 1);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const binding_group = [];
	let value = $.prop($$props, "value", 11, "");
	let group = $.state($.proxy([]));
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	input.value = input.__value = "a";
	var input_1 = $.sibling(input, 2);
	$.remove_input_defaults(input_1);
	input_1.value = input_1.__value = "b";
	var p = $.sibling(input_1, 2);
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value()));
	$.bind_group(binding_group, [], input, () => $.get(group), ($$value) => $.set(group, $$value));
	$.bind_group(binding_group, [], input_1, () => $.get(group), ($$value) => $.set(group, $$value));
	$.append($$anchor, fragment);
	$.pop();
}
