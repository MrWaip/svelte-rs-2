import * as $ from "svelte/internal/client";
var root = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let ops = $.proxy([{
		args: [{
			value: [],
			options: [{ value: "a" }]
		}],
		id: 1
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => ops, ({ args, id }) => id, ($$anchor, $$item, $$index_2) => {
		let args = () => $.get($$item).args;
		let id = () => $.get($$item).id;
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 17, args, $.index, ($$anchor, arg, $$index_1) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.each(node_2, 17, () => $.get(arg).options, $.index, ($$anchor, $$item) => {
				let value = () => $.get($$item).value;
				var input = root();
				$.remove_input_defaults(input);
				var input_value;
				$.template_effect(() => {
					if (input_value !== (input_value = value())) {
						input.value = (input.__value = value()) ?? "";
					}
				});
				$.bind_group(binding_group, [$$index_1, $$index_2], input, () => {
					value();
					return $.get(arg).value;
				}, ($$value) => $.get(arg).value = $$value);
				$.append($$anchor, input);
			});
			$.append($$anchor, fragment_2);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
