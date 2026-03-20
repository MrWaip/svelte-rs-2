import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<input type="checkbox"/>`);
export default function App($$anchor) {
	const binding_group = [];
	let categories = $.proxy([{
		id: 1,
		name: "fruit",
		selected: []
	}, {
		id: 2,
		name: "veg",
		selected: []
	}]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => categories, (category) => category.id, ($$anchor, category, $$index) => {
		var input = root_1();
		$.remove_input_defaults(input);
		input.value = input.__value = "apple";
		$.bind_group(binding_group, [$$index], input, () => $.get(category).selected, ($$value) => $.get(category).selected = $$value);
		$.append($$anchor, input);
	});
	$.append($$anchor, fragment);
}
