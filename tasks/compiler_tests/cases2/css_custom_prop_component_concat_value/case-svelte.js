import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<div><svelte-css-wrapper style="display: contents"><!></svelte-css-wrapper></div> <button>x</button>`, 1);
export default function App($$anchor) {
	let val = $.state("25");
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
	$.delegated("click", button, () => $.set(val, "50"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
