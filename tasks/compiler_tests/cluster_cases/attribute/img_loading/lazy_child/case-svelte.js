import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<h1>t</h1> <img src="x" loading="lazy"/>`, 1);
export default function App($$anchor) {
	var fragment = root();
	var img = $.sibling($.first_child(fragment), 2);
	$.append($$anchor, fragment);
}
