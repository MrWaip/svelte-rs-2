import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let selected_array = $.prop($$props, "selected_array", 8);
	let selected_index = $.prop($$props, "selected_index", 8);
	let options = $.prop($$props, "options", 8);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, selected_array, $.index, ($$anchor, selected, $$index_2) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 1, selected_index, $.index, ($$anchor, index, $$index_1) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.each(node_2, 1, options, $.index, ($$anchor, value) => {
				var input = root();
				$.remove_input_defaults(input);
				var input_value;
				$.template_effect(() => {
					if (input_value !== (input_value = $.get(value))) {
						input.value = (input.__value = $.get(value)) ?? "";
					}
				});
				$.bind_group(binding_group, [$$index_1, $$index_2], input, () => {
					$.get(value);
					return $.get(selected)[$.get(index)];
				}, ($$value) => ($.get(selected)[$.get(index)] = $$value, $.invalidate_inner_signals(() => selected_array())));
				$.append($$anchor, input);
			});
			$.append($$anchor, fragment_2);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
