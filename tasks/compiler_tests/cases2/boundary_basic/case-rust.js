import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p>hello</p>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		var p = root_1();
		$.append($$anchor, p);
	});
	$.append($$anchor, fragment);
}
