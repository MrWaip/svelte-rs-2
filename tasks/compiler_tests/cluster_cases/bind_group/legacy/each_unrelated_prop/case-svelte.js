import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let selected = $.prop($$props, "selected", 12);
	let values = $.prop($$props, "values", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, values, $.index, ($$anchor, value) => {
		var input = root_1();
		$.remove_input_defaults(input);
		var input_value;
		$.template_effect(() => {
			if (input_value !== (input_value = $.get(value))) {
				input.value = (input.__value = $.get(value)) ?? "";
			}
		});
		$.bind_group(binding_group, [], input, () => {
			$.get(value);
			return selected();
		}, selected);
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
