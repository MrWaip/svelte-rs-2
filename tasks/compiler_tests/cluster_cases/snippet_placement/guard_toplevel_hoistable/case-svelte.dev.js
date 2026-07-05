App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const t = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var p = root();
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p>hi</p>`), App[$.FILENAME], [[3, 14]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => t($$anchor), "render", App, 1, 0);
	return $.pop($$exports);
}
