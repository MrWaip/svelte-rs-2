import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	function myFailed($$anchor, error) {
		console.log(error);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { failed: myFailed }, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
