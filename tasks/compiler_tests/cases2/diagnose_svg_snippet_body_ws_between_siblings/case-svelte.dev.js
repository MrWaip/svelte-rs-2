App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const shape = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var fragment = root();
	$.next();
	$.append($$anchor, fragment);
});
var root = $.add_locations($.from_svg(`<g><path d="M1"></path></g><g><path d="M2"></path></g>`, 1), App[$.FILENAME], [[
	2,
	1,
	[[2, 4]]
], [
	3,
	1,
	[[3, 4]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => shape($$anchor), "render", App, 6, 0);
	return $.pop($$exports);
}
