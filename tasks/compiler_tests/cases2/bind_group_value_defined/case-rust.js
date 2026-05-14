import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="radio"/> <button>rotate</button>`, 1);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let initial = $.prop($$props, "initial", 3, "a");
	let selected = $.state(null);
	let dyn_val = $.state($.proxy(initial()));
	function rotate() {
		$.set(dyn_val, $.get(dyn_val) + "!");
	}
	var fragment = root();
	var input = $.first_child(fragment);
	$.remove_input_defaults(input);
	var input_value;
	var button = $.sibling(input, 2);
	$.template_effect(() => {
		if (input_value !== (input_value = `item-${$.get(dyn_val)}`)) {
			input.value = input.__value = `item-${$.get(dyn_val)}`;
		}
	});
	$.bind_group(binding_group, [], input, () => {
		`item-${$.get(dyn_val)}`;
		return $.get(selected);
	}, ($$value) => $.set(selected, $$value));
	$.delegated("click", button, rotate);
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
