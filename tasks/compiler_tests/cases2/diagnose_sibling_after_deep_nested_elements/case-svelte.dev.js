App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><div><div><div><span>a</span></div></div></div> <div><!></div></div>`), App[$.FILENAME], [[
	6,
	4,
	[[
		7,
		8,
		[[
			8,
			12,
			[[
				9,
				16,
				[[9, 21]]
			]]
		]]
	], [12, 8]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const inner = $.wrap_snippet(App, function($$anchor, mf = $.noop) {
		$.validate_snippet_args(...arguments);
		var div = root();
		var div_1 = $.sibling($.child(div), 2);
		var node = $.child(div_1);
		$.add_svelte_meta(() => $.key(node, () => x, ($$anchor) => {
			var fragment = $.comment();
			var node_1 = $.first_child(fragment);
			$.add_svelte_meta(() => $.snippet(node_1, mf), "render", App, 14, 16);
			$.append($$anchor, fragment);
		}), "key", App, 13, 12);
		$.reset(div_1);
		$.reset(div);
		$.append($$anchor, div);
	});
	let x = 0;
	var $$exports = { ...$.legacy_api() };
	return $.pop($$exports);
}
