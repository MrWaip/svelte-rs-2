App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>loading</span>`), App[$.FILENAME], [[4, 21]]);
var root_1 = $.add_locations($.from_html(`<p>a</p>`), App[$.FILENAME], [[2, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		const pending = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var span = root();
			$.append($$anchor, span);
		});
		$.boundary(node, { pending }, ($$anchor) => {
			var p = root_1();
			$.append($$anchor, p);
		});
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
