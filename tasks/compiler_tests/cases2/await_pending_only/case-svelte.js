import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>Loading...</p>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
