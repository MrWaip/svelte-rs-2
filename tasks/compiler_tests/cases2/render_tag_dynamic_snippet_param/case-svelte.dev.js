App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const wrapper = $.wrap_snippet(App, function($$anchor, inner = $.noop) {
	$.validate_snippet_args(...arguments);
	var div = root();
	var node = $.child(div);
	$.add_svelte_meta(() => $.snippet(node, inner), "render", App, 6, 6);
	$.reset(div);
	$.append($$anchor, div);
});
const greeting = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var p = root_1();
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[6, 1]]);
var root_1 = $.add_locations($.from_html(`<p>Hello</p>`), App[$.FILENAME], [[10, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let msg = "hi";
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => wrapper($$anchor, () => greeting), "render", App, 13, 0);
	return $.pop($$exports);
}
