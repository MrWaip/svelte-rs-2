import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const row = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var p = root();
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p class="svelte-5iy3wu">hi</p>`), App[$.FILENAME], [[3, 16]]);
var root_1 = $.add_locations($.from_html(`<div class="wrap svelte-5iy3wu"><!></div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	var node = $.child(div);
	$.add_svelte_meta(() => row(node), "render", App, 1, 18);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
