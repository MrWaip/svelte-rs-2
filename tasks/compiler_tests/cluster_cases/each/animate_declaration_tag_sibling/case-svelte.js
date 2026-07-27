import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	function flip() {}
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 24, () => items, (n) => n, ($$anchor, n) => {
		const a = n;
		var div = root();
		$.animation(div, () => flip, () => a);
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
