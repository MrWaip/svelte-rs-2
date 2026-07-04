App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const one = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	$.add_svelte_meta(() => two($$anchor), "render", App, 8, 1);
});
const two = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	$.next();
	var text = $.text();
	text.nodeValue = "hello";
	$.append($$anchor, text);
});
const message = "hello";
export { one };
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
