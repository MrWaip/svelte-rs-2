App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<input type="checkbox"/>`), App[$.FILENAME], [[9, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	let categories = $.tag_proxy($.proxy([{
		id: 1,
		name: "fruit",
		selected: []
	}, {
		id: 2,
		name: "veg",
		selected: []
	}]), "categories");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => categories, (category) => category.id, ($$anchor, category, $$index) => {
		var input = root();
		$.remove_input_defaults(input);
		$.validate_binding("bind:group={category.selected}", [], () => $.get(category), () => "selected", 9, 24);
		input.value = input.__value = "apple";
		$.bind_group(binding_group, [$$index], input, function get() {
			return $.get(category).selected;
		}, function set($$value) {
			$.get(category).selected = $$value;
		});
		$.append($$anchor, input);
	}), "each", App, 8, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
