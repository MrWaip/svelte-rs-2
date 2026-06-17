import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_3 = $.from_html(`<input type="radio"/>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let pipelineOperations = $.prop($$props, "pipelineOperations", 24, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, pipelineOperations, ({ operation, id }) => id, ($$anchor, $$item, $$index_2) => {
		let operation = () => $.get($$item).operation;
		let id = () => $.get($$item).id;
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.each(node_1, 1, () => (operation(), $.untrack(() => operation().args)), $.index, ($$anchor, arg, $$index_1) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.each(node_2, 1, () => ($.get(arg), $.untrack(() => $.get(arg).options)), $.index, ($$anchor, $$item) => {
				let value = () => $.get($$item).value;
				var input = root_3();
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
				}, ($$value) => ($.get(arg).value = $$value, $.invalidate_inner_signals(() => (operation(), pipelineOperations()))));
				$.append($$anchor, input);
			});
			$.append($$anchor, fragment_2);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
}
