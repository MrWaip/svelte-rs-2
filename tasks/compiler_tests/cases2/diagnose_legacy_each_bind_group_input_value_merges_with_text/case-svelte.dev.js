import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<label><span> </span> <input type="radio"/></label>`), App[$.FILENAME], [[
	8,
	1,
	[[9, 2], [10, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	const binding_group = [];
	let items = $.prop($$props, "items", 8);
	let value = $.prop($$props, "value", 12);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, items, (item) => item.value, ($$anchor, item) => {
		var label = root();
		var span = $.child(label);
		var text = $.child(span, true);
		$.reset(span);
		var input = $.sibling(span, 2);
		$.remove_input_defaults(input);
		var input_value;
		$.reset(label);
		$.template_effect(() => {
			$.set_text(text, ($.get(item), $.untrack(() => $.get(item).label)));
			if (input_value !== (input_value = ($.get(item), $.untrack(() => $.get(item).value)))) {
				input.value = (input.__value = ($.get(item), $.untrack(() => $.get(item).value))) ?? "";
			}
		});
		$.bind_group(binding_group, [], input, () => {
			$.get(item), $.untrack(() => $.get(item).value);
			return value();
		}, function set($$value) {
			value($$value);
		});
		$.append($$anchor, label);
	}), "each", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
