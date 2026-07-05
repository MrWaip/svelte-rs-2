import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[9, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let selected_array = $.prop($$props, "selected_array", 8);
	let values = $.prop($$props, "values", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, selected_array, $.index, ($$anchor, selected, $$index_1) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 1, values, $.index, ($$anchor, value) => {
			var input = root();
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
			}, function set($$value) {
				selected_array()[$$index_1] = $$value, $.invalidate_inner_signals(() => selected_array());
			});
			$.append($$anchor, input);
		}), "each", App, 8, 1);
		$.append($$anchor, fragment_1);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
