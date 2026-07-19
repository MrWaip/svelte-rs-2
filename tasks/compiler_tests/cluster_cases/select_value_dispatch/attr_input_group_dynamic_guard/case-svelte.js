import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/> <button>swap</button>`, 1);
export default function App($$anchor) {
	const binding_group = [];
	let group = $.state($.proxy([]));
	let v = $.state("x");
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_value;
	var button = $.sibling(input, 2);
	$.template_effect(() => {
		if (input_value !== (input_value = $.get(v))) {
			input.value = (input.__value = $.get(v)) ?? "";
		}
	});
	$.bind_group(binding_group, [], input, () => {
		$.get(v);
		return $.get(group);
	}, ($$value) => $.set(group, $$value));
	$.delegated("click", button, () => $.set(v, "y"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
