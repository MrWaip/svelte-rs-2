import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let selected_array = $.prop($$props, "selected_array", 8);
	let values = $.prop($$props, "values", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, selected_array, $.index, ($$anchor, selected, $$index_1) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 1, values, $.index, ($$anchor, value) => {
			var input = root_2();
			$.remove_input_defaults(input);
			var input_value;
			$.template_effect(() => {
				if (input_value !== (input_value = $.get(value))) {
					input.value = (input.__value = $.get(value)) ?? "";
				}
			});
			$.bind_group(binding_group, [$$index_1], input, () => {
				$.get(value);
				return selected_array()[$$index_1];
			}, ($$value) => (selected_array()[$$index_1] = $$value, $.invalidate_inner_signals(() => selected_array())));
			$.append($$anchor, input);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
