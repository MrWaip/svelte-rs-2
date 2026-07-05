App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>loading...</p>`), App[$.FILENAME], [[5, 2]]);
var root_1 = $.add_locations($.from_html(`<p>content</p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var p = root();
			$.append($$anchor, p);
		});
		$.boundary(node, { pending }, ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
