import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>Done</p>`);
var root_2 = $.from_html(`<p>Error</p>`);
var root_3 = $.from_html(`<p>Loading</p>`);
export default function App($$anchor) {
	const promise = fetch("/api");
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.await(node, () => promise, ($$anchor) => {
		var p_2 = root_3();
		$.append($$anchor, p_2);
	}, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	}, ($$anchor) => {
		var p_1 = root_2();
		$.append($$anchor, p_1);
	});
	$.append($$anchor, fragment);
}
