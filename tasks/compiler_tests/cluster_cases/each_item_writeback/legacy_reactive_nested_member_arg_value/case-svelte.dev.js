import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="radio"/>`), App[$.FILENAME], [[9, 3]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let pipelineOperations = $.prop($$props, "pipelineOperations", 24, () => []);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, pipelineOperations, ({ operation, id }) => id, ($$anchor, $$item, $$index_2) => {
		let operation = () => $.get($$item).operation;
		operation();
		let id = () => $.get($$item).id;
		id();
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 1, () => (operation(), $.untrack(() => operation().args)), $.index, ($$anchor, arg, $$index_1) => {
			var fragment_2 = $.comment();
			var node_2 = $.first_child(fragment_2);
			$.add_svelte_meta(() => $.each(node_2, 1, () => ($.get(arg), $.untrack(() => $.get(arg).options)), $.index, ($$anchor, $$item) => {
				let value = () => $.get($$item).value;
				value();
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
				}, function set($$value) {
					$.get(arg).value = $$value, $.invalidate_inner_signals(() => (operation(), pipelineOperations()));
				});
				$.append($$anchor, input);
			}), "each", App, 8, 2);
			$.append($$anchor, fragment_2);
		}), "each", App, 7, 1);
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
