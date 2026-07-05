import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor, $$props) {
	let items = $.prop($$props, "items", 19, () => []);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, items, $.index, ($$anchor, item, i) => {
		var div = root();
		div.textContent = i;
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
