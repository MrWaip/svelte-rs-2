import * as $ from "svelte/internal/client";
var root = $.from_html(`<div></div>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, $$item, rank) => {
		var div = root();
		div.textContent = rank;
		$.append($$anchor, div);
	});
	$.append($$anchor, fragment);
}
