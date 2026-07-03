import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	let handler = (error) => console.error(error);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get onerror() {
		return handler;
	} }, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
