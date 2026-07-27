App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var optgroup_content = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var option_content = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var root = $.add_locations($.from_html(`<select><!></select> <select><optgroup label="g"><!></optgroup></select> <select><option><!></option></select>`, 1), App[$.FILENAME], [
	[5, 0],
	[
		6,
		0,
		[[6, 8]]
	],
	[
		7,
		0,
		[[7, 8]]
	]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var select = $.first_child(fragment);
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment_1 = select_content();
		var node = $.first_child(fragment_1);
		$.add_svelte_meta(() => $.snippet(node, () => $$props.r), "render", App, 5, 8);
		$.append(anchor, fragment_1);
	});
	var select_1 = $.sibling(select, 2);
	var optgroup = $.child(select_1);
	$.customizable_select(optgroup, () => {
		var anchor_1 = $.child(optgroup);
		var fragment_2 = optgroup_content();
		var node_1 = $.first_child(fragment_2);
		$.add_svelte_meta(() => $.snippet(node_1, () => $$props.r), "render", App, 6, 28);
		$.append(anchor_1, fragment_2);
	});
	$.reset(select_1);
	var select_2 = $.sibling(select_1, 2);
	var option = $.child(select_2);
	$.customizable_select(option, () => {
		var anchor_2 = $.child(option);
		var fragment_3 = option_content();
		var node_2 = $.first_child(fragment_3);
		$.add_svelte_meta(() => $.snippet(node_2, () => $$props.r), "render", App, 7, 16);
		$.append(anchor_2, fragment_3);
	});
	$.reset(select_2);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
