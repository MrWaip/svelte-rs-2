App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<style>html { height: 100%; }</style>`), App[$.FILENAME], [[9, 4]]);
var root_1 = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	$.head("q2w0q4", ($$anchor) => {
		var style = root();
		$.append($$anchor, style);
	});
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => A(node, {}), "component", App, 5, 0, { componentTag: "A" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.snippet(node_1, () => $$props.children ?? $.noop), "render", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
