App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>a</p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const failed = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			$.next();
			var text = $.text("failed text");
			$.append($$anchor, text);
		});
		$.boundary(node, { failed }, ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
