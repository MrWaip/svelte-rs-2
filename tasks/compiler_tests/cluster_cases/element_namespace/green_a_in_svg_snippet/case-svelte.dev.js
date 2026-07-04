App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<a><text>Hello</text></a>`), App[$.FILENAME], [[
	1,
	19,
	[[1, 22]]
]]);
var root_1 = $.add_locations($.from_svg(`<svg><!></svg>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root_1();
	{
		const s = $.wrap_snippet(App, function($$anchor) {
			$.validate_snippet_args(...arguments);
			var a = root();
			$.append($$anchor, a);
		});
		var node = $.child(svg);
		$.add_svelte_meta(() => s(node), "render", App, 1, 54);
		$.reset(svg);
	}
	$.append($$anchor, svg);
	return $.pop($$exports);
}
