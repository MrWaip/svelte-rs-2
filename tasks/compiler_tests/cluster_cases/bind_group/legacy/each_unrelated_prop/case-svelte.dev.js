import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let selected = $.prop($$props, "selected", 12);
	let values = $.prop($$props, "values", 8);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, values, $.index, ($$anchor, value) => {
		var input = root();
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
		}, function set($$value) {
			selected($$value);
		});
		$.append($$anchor, input);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
