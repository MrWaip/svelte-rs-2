App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const defaultWrapWith = $.wrap_snippet(App, function($$anchor, mf = $.noop) {
	$.validate_snippet_args(...arguments);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.snippet(node, mf), "render", App, 8, 1);
	$.append($$anchor, fragment);
});
var root = $.add_locations($.from_html(`<div>x</div>`), App[$.FILENAME], [[11, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let wrapWith = $.prop($$props, "wrapWith", 3, defaultWrapWith);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
