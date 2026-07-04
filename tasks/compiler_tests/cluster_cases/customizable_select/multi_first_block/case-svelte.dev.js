App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.from_html(`<!><div>x</div><!>`, 1), App[$.FILENAME], [[5, 26]]);
var root = $.add_locations($.from_html(`<select><!></select>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var node = $.first_child(fragment);
		$.add_svelte_meta(() => $.snippet(node, () => $$props.header), "render", App, 5, 8);
		var node_1 = $.sibling(node, 2);
		$.add_svelte_meta(() => $.snippet(node_1, () => $$props.footer), "render", App, 5, 38);
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
