import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>content</p>`);
export default function App($$anchor) {
	function pending($$anchor) {
		console.log($$anchor);
	}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { pending }, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
