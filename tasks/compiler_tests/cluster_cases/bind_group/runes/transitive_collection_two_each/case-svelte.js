import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let list = $.proxy([{ data: [{ value: [] }] }]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => list, $.index, ($$anchor, $$item, $$index_1) => {
		let data = () => $.get($$item).data;
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 17, data, $.index, ($$anchor, item, $$index) => {
			var input = root();
			$.remove_input_defaults(input);
			input.value = input.__value = "a";
			$.bind_group(binding_group, [$$index, $$index_1], input, () => $.get(item).value, ($$value) => $.get(item).value = $$value);
			$.append($$anchor, input);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
