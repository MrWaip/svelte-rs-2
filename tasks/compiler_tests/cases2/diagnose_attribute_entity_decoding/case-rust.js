import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<div title="a b&amp;c&lt;d">x</div> <!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	Child(node, { label: "a\xA0b&c<d" });
	$.append($$anchor, fragment);
}
