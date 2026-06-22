import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<a href="foo/bar">link</a>`);
export default function App($$anchor) {
	var a = root();
	$.append($$anchor, a);
}
