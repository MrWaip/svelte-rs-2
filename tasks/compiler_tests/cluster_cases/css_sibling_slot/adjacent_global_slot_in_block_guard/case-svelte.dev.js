import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><!> <p class="foo">foo</p></div>`), App[$.FILENAME], [[
	1,
	0,
	[[3, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	{
		var consequent = ($$anchor) => {
			var fragment = $.comment();
			var node_1 = $.first_child(fragment);
			$.slot(node_1, $$props, "default", {}, null);
			$.append($$anchor, fragment);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (true) $$render(consequent);
		}), "if", App, 2, 2);
	}
	$.next(2);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
