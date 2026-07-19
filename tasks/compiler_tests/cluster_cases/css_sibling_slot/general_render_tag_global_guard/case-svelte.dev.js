import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><p class="before svelte-105a842">before</p> <!> <p class="foo svelte-105a842"><span class="svelte-105a842">foo</span></p> <p class="bar svelte-105a842">bar</p></div>`), App[$.FILENAME], [[
	1,
	0,
	[
		[2, 1],
		[
			4,
			1,
			[[5, 2]]
		],
		[7, 1]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.sibling($.child(div), 2);
	$.add_svelte_meta(() => $.snippet(node, () => children), "render", App, 3, 1);
	$.next(4);
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
