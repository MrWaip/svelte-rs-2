App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var select_content_1 = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var root = $.add_locations($.from_html(`<select><!></select> <select><!></select>`, 1), App[$.FILENAME], [[5, 0], [6, 0]]);
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
		$.add_svelte_meta(() => $.snippet(node, () => $$props.opt), "render", App, 5, 8);
		$.append(anchor, fragment_1);
	});
	var select_1 = $.sibling(select, 2);
	$.customizable_select(select_1, () => {
		var anchor_1 = $.child(select_1);
		var fragment_2 = select_content_1();
		var node_1 = $.first_child(fragment_2);
		$.add_svelte_meta(() => $.snippet(node_1, () => $$props.opt), "render", App, 6, 8);
		$.append(anchor_1, fragment_2);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
