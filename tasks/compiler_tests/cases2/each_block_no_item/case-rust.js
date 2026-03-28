import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<div>item</div>`);
export default function App($$anchor) {
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 16, () => items, $.index, ($$anchor, $$item) => {
		var div = root_1();
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
