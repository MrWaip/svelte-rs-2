import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let items = [{ id: "a" }, { id: "b" }];
	let selected = $.mutable_source();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, item) => {
		var input = root_1();
		$.remove_input_defaults(input);
		var input_value;
		$.template_effect(() => {
			if (input_value !== (input_value = ($.get(item), $.untrack(() => $.get(item).id)))) {
				input.value = (input.__value = ($.get(item), $.untrack(() => $.get(item).id))) ?? "";
			}
		});
		$.bind_group(binding_group, [], input, () => {
			$.get(item), $.untrack(() => $.get(item).id);
			return $.get(selected);
		}, ($$value) => $.set(selected, $$value));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
