App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let items = $.tag_proxy($.proxy([
		"a",
		"b",
		"c"
	]), "items");
	let selected = $.tag($.state($.proxy([])), "selected");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, item) => {
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
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
