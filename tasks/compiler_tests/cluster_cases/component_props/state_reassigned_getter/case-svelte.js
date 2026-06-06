import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<button>x</button> <!>`, 1);
export default function App($$anchor) {
	let x = $.state("x1");
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	Child(node, { get x() {
		return $.get(x);
	} });
	$.delegated("click", button, () => $.set(x, "x2"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
