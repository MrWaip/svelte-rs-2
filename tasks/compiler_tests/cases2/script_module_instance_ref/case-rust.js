import * as $ from "svelte/internal/client";
const BASE = "https://example.com";
var root = $.from_html(`<a>Link</a>`);
export default function App($$anchor) {
	let path = "/home";
	let url = $.derived(() => BASE + path);
	var a = root();
	$.set_attribute(a, "href", $.get(url));
	$.append($$anchor, a);
}
