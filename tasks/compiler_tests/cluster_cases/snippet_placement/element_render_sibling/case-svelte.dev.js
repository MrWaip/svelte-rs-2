App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span>hi</span>`), App[$.FILENAME], [[3, 2]]);
var root_1 = $.add_locations($.from_html(`<div><!></div>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root_1();
	{
		const t = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var span = root();
			$.append($$anchor, span);
		});
		var node = $.child(div);
		$.add_svelte_meta(() => t(node), "render", App, 5, 1);
		$.reset(div);
	}
	$.append($$anchor, div);
	return $.pop($$exports);
}
