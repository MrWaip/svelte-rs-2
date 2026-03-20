import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let groups = $.proxy([["a", "b"], ["c", "d"]]);
	let selected = $.state($.proxy([]));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => groups, $.index, ($$anchor, group) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 17, () => $.get(group), $.index, ($$anchor, item) => {
			var input = root_2();
			$.remove_input_defaults(input);
			var input_value;
			$.template_effect(() => {
				if (input_value !== (input_value = $.get(item))) {
					input.value = (input.__value = $.get(item)) ?? "";
				}
			});
			$.bind_group(binding_group, [], input, () => {
				$.get(item);
				return $.get(selected);
			}, ($$value) => $.set(selected, $$value));
			$.append($$anchor, input);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
