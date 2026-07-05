App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const a = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		$.next();
		var text = $.text();
		text.nodeValue = "a";
		$.append($$anchor, text);
	});
	const b = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		$.add_svelte_meta(() => a($$anchor), "render", App, 12, 1);
	});
	let abc = "a";
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => b($$anchor), "render", App, 5, 0);
	return $.pop($$exports);
}
