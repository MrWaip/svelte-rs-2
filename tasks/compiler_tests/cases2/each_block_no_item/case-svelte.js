import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>item</div>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, $$item) => {
		var div = root();
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
