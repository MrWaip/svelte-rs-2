import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<label><span> </span> <input type="radio"/></label>`);
export default function App($$anchor, $$props) {
	const binding_group = [];
	let items = $.prop($$props, "items", 8);
	let value = $.prop($$props, "value", 12);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, items, (item) => item.value, ($$anchor, item) => {
		var label = root_1();
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
		}, value);
		$.append($$anchor, label);
	});
	$.append($$anchor, fragment);
}
