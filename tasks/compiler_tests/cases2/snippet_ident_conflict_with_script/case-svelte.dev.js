App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const card = $.wrap_snippet(App, function($$anchor, heading = $.noop) {
	$.validate_snippet_args(...arguments);
	var div = root();
	var h3 = $.child(div);
	var text = $.child(h3, true);
	$.reset(h3);
	var node_1 = $.sibling(h3, 2);
	$.add_svelte_meta(() => $.snippet(node_1, () => badge, () => "new"), "render", App, 10, 8);
	$.reset(div);
	$.template_effect(() => $.set_text(text, heading()));
	$.append($$anchor, div);
});
var root = $.add_locations($.from_html(`<div><h3> </h3> <!></div>`), App[$.FILENAME], [[
	8,
	4,
	[[9, 8]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function action(node, arg) {
		return { destroy() {} };
	}
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
