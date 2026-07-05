App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const greeting = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var p = root();
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p>Hello</p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => greeting?.($$anchor), "render", App, 5, 0);
	return $.pop($$exports);
}
