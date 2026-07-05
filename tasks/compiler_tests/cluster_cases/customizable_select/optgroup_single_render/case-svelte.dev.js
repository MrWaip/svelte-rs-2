App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var optgroup_content = $.add_locations($.from_html(`<!>`, 1), App[$.FILENAME], []);
var root = $.add_locations($.from_html(`<optgroup><!></optgroup>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var optgroup = root();
	$.customizable_select(optgroup, () => {
		var anchor = $.child(optgroup);
		var fragment = optgroup_content();
		var node = $.first_child(fragment);
		$.add_svelte_meta(() => $.snippet(node, () => $$props.children), "render", App, 5, 10);
		$.append(anchor, fragment);
	});
	$.append($$anchor, optgroup);
	return $.pop($$exports);
}
