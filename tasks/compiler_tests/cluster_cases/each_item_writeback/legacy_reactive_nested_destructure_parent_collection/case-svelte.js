import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_2 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let list = [{
		id: "x",
		data: [{
			id: 1,
			data: []
		}]
	}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => list, $.index, ($$anchor, $$item, $$index_1) => {
		let id = () => $.get($$item).id;
		let data = () => $.get($$item).data;
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 1, data, $.index, ($$anchor, item, $$index) => {
			var input = root_2();
			$.remove_input_defaults(input);
			input.value = input.__value = "a";
			$.template_effect(() => $.set_attribute(input, "data-index", `${id() ?? ""}-${($.get(item), $.untrack(() => $.get(item).id)) ?? ""}`));
			$.bind_group(binding_group, [$$index, $$index_1], input, () => $.get(item).data, ($$value) => ($.get(item).data = $$value, $.invalidate_inner_signals(() => (data(), list))));
			$.append($$anchor, input);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
