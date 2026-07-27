import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div> <my-thing></my-thing>`, 3);
export default function App($$anchor) {
	var fragment = root();
	var my_thing = $.sibling($.first_child(fragment), 2);
	$.append($$anchor, fragment);
}
