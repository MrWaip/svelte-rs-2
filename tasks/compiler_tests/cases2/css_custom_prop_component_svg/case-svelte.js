import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_svg(`<g><!></g>`, 1);
export default function App($$anchor) {
	let color = "red";
	var fragment = root();
	var node = $.first_child(fragment);
	{
		$.css_props(node, () => ({ "--color": color }));
		Child(node.lastChild, {});
		$.reset(node);
	}
	$.append($$anchor, fragment);
}
