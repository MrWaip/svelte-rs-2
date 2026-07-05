App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<div><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></div> <button>x</button>`, 1), App[$.FILENAME], [[
	6,
	0,
	[[7, 2]]
], [9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let val = $.tag($.state("25"), "val");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var node = $.child(div);
	{
		$.css_props(node, () => ({ "--color": `px ${$.get(val) ?? ""}` }));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.reset(div);
	var button = $.sibling(div, 2);
	$.delegated("click", button, function click() {
		return $.set(val, "50");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
