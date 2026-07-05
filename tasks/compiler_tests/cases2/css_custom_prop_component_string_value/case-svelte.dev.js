App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></div>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 2]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var node = $.child(div);
	{
		$.css_props(node, () => ({ "--color": "25%" }));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.reset(div);
	$.append($$anchor, div);
	return $.pop($$exports);
}
