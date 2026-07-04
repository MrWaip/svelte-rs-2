App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const t = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		$.next();
		var text = $.text();
		text.nodeValue = "x";
		$.append($$anchor, text);
	});
	let name = "x";
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => t($$anchor), "render", App, 5, 0);
	return $.pop($$exports);
}
