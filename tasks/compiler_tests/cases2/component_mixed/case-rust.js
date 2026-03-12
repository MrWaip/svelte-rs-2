import * as $ from "svelte/internal/client";
import Button from "./Button.svelte";
import Icon from "./Icon.svelte";
var root = $.from_html(`<h1>Title</h1> <!> <!>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var node = $.sibling($.first_child(fragment), 2);
	Button(node, {});
	var node_1 = $.sibling(node, 2);
	Icon(node_1, {});
	$.append($$anchor, fragment);
}
