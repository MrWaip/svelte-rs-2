import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<div><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></div>`);
export default function App($$anchor) {
	let color = "red";
	var div = root();
	var node = $.child(div);
	{
		$.css_props(node, () => ({ "--color": color }));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.reset(div);
	$.append($$anchor, div);
}
