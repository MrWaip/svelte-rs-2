import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let items = $.proxy([{
		id: 1,
		name: "a"
	}, {
		id: 2,
		name: "b"
	}]);
	let selected = $.state($.proxy([]));
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, (item) => item.id, ($$anchor, item) => {
		var input = root_1();
		$.remove_input_defaults(input);
		var input_value;
		$.template_effect(() => {
			if (input_value !== (input_value = $.get(item).name)) {
				input.value = (input.__value = $.get(item).name) ?? "";
			}
		});
		$.bind_group(binding_group, [], input, () => {
			$.get(item).name;
			return $.get(selected);
		}, ($$value) => $.set(selected, $$value));
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
