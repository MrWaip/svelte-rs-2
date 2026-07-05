App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[8, 2]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let groups = $.tag_proxy($.proxy([["a", "b"], ["c", "d"]]), "groups");
	let selected = $.tag($.state($.proxy([])), "selected");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => groups, $.index, ($$anchor, group) => {
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.each(node_1, 17, () => $.get(group), $.index, ($$anchor, item) => {
			var input = root();
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
			}, function set($$value) {
				$.set(selected, $$value);
			});
			$.append($$anchor, input);
		}), "each", App, 7, 1);
		$.append($$anchor, fragment_1);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
