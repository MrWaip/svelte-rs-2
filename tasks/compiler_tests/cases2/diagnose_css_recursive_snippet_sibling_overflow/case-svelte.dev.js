App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const a = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => b(node), "render", App, 2, 4);
	var div = $.sibling(node, 2);
	var node_1 = $.child(div);
	$.add_svelte_meta(() => b(node_1), "render", App, 4, 8);
	$.reset(div);
	$.append($$anchor, fragment);
});
const b = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var fragment_1 = root_1();
	var node_2 = $.first_child(fragment_1);
	$.add_svelte_meta(() => a(node_2), "render", App, 9, 4);
	var div_1 = $.sibling(node_2, 2);
	var node_3 = $.child(div_1);
	$.add_svelte_meta(() => a(node_3), "render", App, 11, 8);
	$.reset(div_1);
	$.append($$anchor, fragment_1);
});
var root = $.add_locations($.from_html(`<!> <div class="svelte-ile0r9"><!></div>`, 1), App[$.FILENAME], [[3, 4]]);
var root_1 = $.add_locations($.from_html(`<!> <div class="svelte-ile0r9"><!></div>`, 1), App[$.FILENAME], [[10, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
